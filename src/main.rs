#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

pub mod drivers;
use x86_64;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Helllooooo :)");
	drivers::init();
	
	// x86_64::instructions::interrupts::int3();

    println!("No crash");

    drivers::hlt_loop()
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n{}", _info);

    drivers::hlt_loop()
}


