#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

pub mod drivers;
use x86_64;
use core::str;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kmain);

#[no_mangle] // don't mangle the name of this function
fn kmain(boot_info: &'static BootInfo) -> ! {
	use x86_64::{structures::paging::Page, VirtAddr};
	use x86_64::{structures::paging::MapperAllSizes};

	drivers::init();
    drivers::term::TERM.lock().cls();
	
//	x86_64::instructions::interrupts::int3();

    print!("user@proj-r$ ");

    let temp: [u8; 4];
    temp = drivers::interrupts::input_from_user();
    println!("\n{}", str::from_utf8(&temp).unwrap());

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

	// let mapper = unsafe { drivers::memory::init(phys_mem_offset) };

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    //println!("~~~~ [TIMER DEMO] ~~~~\n");
    //println!("Task: We will try to handle the timer interrupt:- ");
    // println!("[Printing] Vga Display is working correctly\n");

    // let test = (1,2,3);
    // println!(">> Trying to use formatting macro to print a struct\n{:?}", test);

    // println!("\n Trying to access illegal memory by printing to wrong offset");
    
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //    // let phys = unsafe { drivers::memory::translate_addr(virt, phys_mem_offset) };
	// 	let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    drivers::hlt_loop()
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n{}", _info);

    drivers::hlt_loop()
}


