use crate::{print, println};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// IDT must have a static lifetime since it will need to be referenced by the
// CPU even when the variable goes out of scope. Thus, use lazy_static macro

// start the interrupt vector numbers from base 32 so they don't overlap
// with CPU exceptions
pub const PIC1_BASE: u8 = 32;
// PIC 1 has 8 lines therefore PIC2_BASE would be 32+8
pub const PIC2_BASE: u8 = 32 + 8;

pub const KEY_BUF_SIZE: usize = 1024;

#[derive(Debug)]
pub struct InputStruct {
    input_mode_flag: bool,
    input_ready_flag: bool,
    input_ptr_start: usize,
    input_ptr_end: usize,
}

lazy_static! {
    pub static ref KEY_BUF: spin::Mutex<[u8; KEY_BUF_SIZE]> =
        spin::Mutex::new([0 as u8; KEY_BUF_SIZE]);
    pub static ref KEY_BUF_INDEX: spin::Mutex<usize> = spin::Mutex::new(0 as usize);
    pub static ref INPUT_STRUCT: spin::Mutex<InputStruct> = spin::Mutex::new(InputStruct {
        input_mode_flag: false,
        input_ready_flag: false,
        input_ptr_start: 0,
        input_ptr_end: 0,
    });
}

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

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_BASE, PIC2_BASE) });

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut InterruptStackFrame) {
    println!("\n[EXCEPTION]: Breakpoint\n{:#?}", _stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("[Exception]: Page Fault");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", _stack_frame);
    crate::drivers::hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("\n[EXCEPTION]: Double Fault\n{:#?}", _stack_frame);
}

lazy_static! {
    pub static ref TIMER_DEMO_FLAG: spin::Mutex<bool> = spin::Mutex::new(false);
    pub static ref TIMER_DEMO_COUNT: spin::Mutex<u8> = spin::Mutex::new(0 as u8);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) {
    if *TIMER_DEMO_FLAG.lock() {
        print!("*");
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(PicIntIndex::Timer.as_u8());
    }
}

pub fn toggle_timer() {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut flag = TIMER_DEMO_FLAG.lock();
        *flag = !*flag
    });
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

extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(ch) => {
                    let mut key_buf = KEY_BUF.lock();
                    let mut i = KEY_BUF_INDEX.lock();
                    let mut input = INPUT_STRUCT.lock();

                    match ch {
                        '\n' => {
                            input.input_ptr_end = *i;
                            input.input_ready_flag = true;
                        }
                        _ => {
                            if input.input_mode_flag == true {
                                input.input_mode_flag = false;
                                input.input_ptr_start = *i;
                            }

                            key_buf[*i] = ch as u8;
                            *i += 1;
                            drop(key_buf);
                            drop(i);

                            print!("{}", ch)
                            // println!("{}", *KEY_BUF_INDEX.lock())
                        }
                    }
                }
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(PicIntIndex::Keyboard.as_u8());
    }
}

pub fn idt_init() {
    IDT.load();
}

pub fn input_from_user() -> [u8; 4] {
    use x86_64::instructions::interrupts;
    // loop till ready

    interrupts::without_interrupts(|| {
        let mut inp = INPUT_STRUCT.lock();
        inp.input_mode_flag = true;
    });

    let mut ready = false;
    while !ready {
        interrupts::without_interrupts(|| {
            ready = INPUT_STRUCT.lock().input_ready_flag;
        });
        for _i in 1..0xffff {}
    }

    interrupts::without_interrupts(|| {
        let mut inp = INPUT_STRUCT.lock();
        inp.input_ready_flag = false;
    });

    let mut temp = [0 as u8; 4];
    interrupts::without_interrupts(|| {
        let key_buf = KEY_BUF.lock();
        let inp_start = INPUT_STRUCT.lock().input_ptr_start;
        for i in 0..4 {
            temp[i] = key_buf[inp_start + i];
        }
    });
    temp
}

lazy_static! {
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
