#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod drivers;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	//drivers::term::term_print("Hi\nEverything seems to work :D");
    println!("- Welcome to project-r. If you see this yellow text, everything is working\n  fine :)");
    println!("- The kernel barely has any features right now but we can already see Rust's\n  advantages in action.");
    println!("- The text on the screen is produced by writing to a buffer which represents\n  a 25x80 character screen");
    println!("- The next instruction will try to write to row 28 (which exceeds the 25 rows\n  limit");
    println!("- Even though such illegal accesses to memory are allowed by C, Rust will \n  instantly detect it in runtime and throw a panic.\n\n");
	drivers::vga::vga_put(b'A', 28, 0);
	
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
