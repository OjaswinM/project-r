use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, print};
use lazy_static::lazy_static;
use spin;
use pic8259_simple::ChainedPics;

// IDT must have a static lifetime since it will need to be referenced by the 
// CPU even when the variable goes out of scope. Thus, use lazy_static macro

// start the interrupt vector numbers from base 32 so they don't overlap
// with CPU exceptions
pub const PIC1_BASE: u8 = 32;
// PIC 1 has 8 lines therefore PIC2_BASE would be 32+8
pub const PIC2_BASE: u8 = 32 + 8;

// Enum for pic8259 interrupt vector table indexes
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PicIntIndex {
	Timer = PIC1_BASE,
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

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
	println!("[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("[EXCEPTION]: Double Fault\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_handler(stack_frame: &mut InterruptStackFrame) {
	print!(". ");

	unsafe {
		PICS.lock().notify_end_of_interrupt(PicIntIndex::Timer.as_u8());
	}
}

pub fn idt_init() {
	IDT.load();
}

lazy_static!{
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(double_fault_handler);
		idt[PicIntIndex::Timer.as_usize()].set_handler_fn(timer_handler);
		idt
	};
}


