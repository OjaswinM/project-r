#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	//drivers::term::term_print("Hi\nEverything seems to work :D");
    println!("Helllooooo :)");
	drivers::vga::vga_put(b'A', 81, 0);
	
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
