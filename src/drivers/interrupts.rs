use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, print};
use lazy_static::lazy_static;

// IDT must have a static lifetime since it will need to be referenced by the 
// CPU even when the variable goes out of scope. Thus, use lazy_static macro

lazy_static!{
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(double_fault_handler);
		idt
	};
}

pub fn idt_init() {
	IDT.load();
}


pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
	println!("[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("[EXCEPTION]: Double Fault\n{:#?}", stack_frame);
}
