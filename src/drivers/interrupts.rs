use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, print};
use lazy_static::lazy_static;
use volatile::Volatile;
use spin;
use pic8259_simple::ChainedPics;
use x86_64::structures::idt::PageFaultErrorCode;

// IDT must have a static lifetime since it will need to be referenced by the 
// CPU even when the variable goes out of scope. Thus, use lazy_static macro

// start the interrupt vector numbers from base 32 so they don't overlap
// with CPU exceptions
pub const PIC1_BASE: u8 = 32;
// PIC 1 has 8 lines therefore PIC2_BASE would be 32+8
pub const PIC2_BASE: u8 = 32 + 8;

pub const KEY_BUF_SIZE: usize = 128;

// Enum for pic8259 interrupt vector table indexes
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PicIntIndex {
	Timer = PIC1_BASE,
	Keyboard,
}

impl PicIntIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
		ChainedPics::new(PIC1_BASE, PIC2_BASE)
	});

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
	println!("\n[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler( stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode,) {
    use x86_64::registers::control::Cr2;

    println!("[Exception]: Page Fault");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
	crate::drivers::hlt_loop();
}


extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("\n[EXCEPTION]: Double Fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(stack_frame: &mut InterruptStackFrame) {
    print!(".");

	unsafe {
		PICS.lock().notify_end_of_interrupt(PicIntIndex::Timer.as_u8());
	}
}

pub static mut KEY_BUF: &'static mut [u8; KEY_BUF_SIZE] = & mut [0 as u8; KEY_BUF_SIZE] ; 

// lazy_static! {
    // pub static ref KEY_BUF_INDEX: usize = 0 as usize;
// }

pub static mut KEY_BUF_INDEX: usize = 0 as usize;

lazy_static!{
    pub static ref READY_FLAG: spin::Mutex<Volatile<bool>> = spin::Mutex::new(Volatile::new(false));
}

extern "x86-interrupt" fn add_to_key_buf(ch: u8) {
//     use::core::str;
// 
//     unsafe { 
//         KEY_BUF[KEY_BUF_INDEX] = ch;
//         KEY_BUF_INDEX += 1;
//         println!("{}", str::from_utf8(KEY_BUF).unwrap());
//     }
// 
        print!("{}", ch);
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut InterruptStackFrame) {
	use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(ch) => {
                    if ch == '\n' {
                        unsafe {READY_FLAG.lock().write(true)}
                    } else {
                         use::core::str;
                         unsafe { 
                             KEY_BUF[KEY_BUF_INDEX] = ch as u8;
                             KEY_BUF_INDEX += 1;
                             //println!("{}", str::from_utf8(&KEY_BUF[0..KEY_BUF_INDEX]).unwrap());
                             print!("{}", ch);
                         }
                    }
                },
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

	unsafe {
		PICS.lock().notify_end_of_interrupt(PicIntIndex::Keyboard.as_u8());
	}
}

pub fn idt_init() {
	IDT.load();
}

pub fn input_from_user() -> [u8; 4] {
    // loop till ready
    unsafe{

       //  while true {
       //      if (READY_FLAG.lock().read()) {
       //          break;
       //      }
       //  }

        use::core::str;

        //let temp = str::from_utf8(KEY_BUF).unwrap().trim();
        //temp = [u8; KEY_BUF_INDEX]

        //for i in 0..KEY_BUF_INDEX {
        //    temp[i] = KEY_BUF[i];
        //    KEY_BUF[i] = 0;
        //}

        //KEY_BUF_INDEX = 0;
        //for i in 0..KEY_BUF_SIZE {
           //KEY_BUF[i] = 0 as u8;
        //}
        let mut temp: [u8; 4] = [0; 4];
        for i in 0..4 {
            temp[i] = KEY_BUF[i];
        }

        READY_FLAG.lock().write(false);
        temp
    }
}

lazy_static!{
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(double_fault_handler);
		idt.page_fault.set_handler_fn(page_fault_handler);
		idt[PicIntIndex::Timer.as_usize()].set_handler_fn(timer_handler);
		idt[PicIntIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
		idt
	};
}


