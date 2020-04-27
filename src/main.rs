#![no_std]
#![no_main]

use core::panic::PanicInfo;

// TODO: Implement a proper panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

// TODO: Implement the kernel _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}


