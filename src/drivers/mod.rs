pub mod vga;
pub mod term;
pub mod interrupts;

pub fn init() {
	interrupts::idt_init();
	unsafe{ interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}
