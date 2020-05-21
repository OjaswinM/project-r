pub mod vga;
pub mod term;
pub mod interrupts;

pub fn init() {
	interrupts::idt_init();
}
