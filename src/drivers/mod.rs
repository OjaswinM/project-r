#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

pub mod interrupts;
pub mod memory;
pub mod term;
pub mod vga;

use x86_64;

pub fn init() {
    interrupts::idt_init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
