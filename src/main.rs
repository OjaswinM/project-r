#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

pub mod drivers;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::str;
use x86_64;

entry_point!(kmain);

#[no_mangle] // don't mangle the name of this function
fn kmain(boot_info: &'static BootInfo) -> ! {
    use drivers::term;
    use x86_64::structures::paging::MapperAllSizes;
    use x86_64::{structures::paging::Page, VirtAddr};

    drivers::init();

    let mut run = 0;

    term::term_cls();
    print_welcome();
    term::print_yellow("Press <Enter> to launch shell");
    pause();

    loop {
        term::term_cls();
        term::print_yellow("user@proj-r$ ");

        let temp: [u8; 4];
        temp = drivers::interrupts::input_from_user();
        let input = str::from_utf8(&temp).unwrap();

        match input {
            "disp" => {
                disp_demo();
            }
            "keyb" => {
                keyboard_demo();
            }
            "time" => {
                timer_demo();
            }
            "excp" => {
                exception_demo();
            }
            "inpt" => {
                input_demo();
            }
            "exit" => {
                break;
            }
            _ => {
                println!("\nError: Command not found. Supported commands are disp, keyb, time, excp, inpt and exit");
                pause();
            }
        }
    }
    drivers::hlt_loop()
}

fn pause() {
    drivers::interrupts::input_from_user();
}

fn disp_demo() {
    println!("\n ~~~~ [DISPLAY DEMO 1] ~~~~\n");
    drivers::term::print_yellow("Task: Demonstrate the working of VGA driver. The VGA driver is a low level driver used to print to a character on a 25x80 screen\n\n");
    print!(" - [Vga Demo] Enter the row: ");

    let inp = drivers::interrupts::input_from_user();
    let temp = str::from_utf8(&inp).unwrap();
    let row: usize = temp[0..2].parse().unwrap();

    print!("\n - [Vga Demo] Enter the column: ");
    let inp = drivers::interrupts::input_from_user();
    let temp = str::from_utf8(&inp[0..2]).unwrap();
    let col: usize = temp.parse().unwrap();

    println!("\n - [Vga Demo] Printing 'X' to given row and column... ");
    drivers::vga::vga_put(b'X', row, col);
    pause();
    println!(" - [Vga Demo] Success, press <Enter> to continue.");
    pause();

    drivers::term::term_cls();
    println!("\n ~~~~ [DISPLAY DEMO 2] ~~~~\n");
    drivers::term::print_yellow("Task: Demonstrate the working of Terminal driver. The Terminal driver is an abstraction over the VGA driver. It is used to print data like strings, structs and tuples to the screen.\n\n");

    print!(" - [Terminal Demo] Trying to interpret newline...");
    print!("\n - [Terminal Demo] Success!\n");
    pause();

    let test = (1, 2, 3);
    println!(" - [Terminal Demo] Trying to use formatting macro to print a tuple.");
    println!(" - [Terminal Demo] {:?}...Working correctly!", test);

    println!("\n - [Terminal Demo] Demo completed! Press <Enter> to exit\n");
    pause();
}

fn keyboard_demo() {
    println!("\n ~~~~ [KEYBOARD DEMO] ~~~~\n");
    drivers::term::print_yellow("Task: Demonstrate the working of keyboard interrupt handler by printing the key pressed on the screen\n\n");
    print!(" - [Keyboard Demo] Enter any key (press <Enter> to end demo): ");
    pause();

    println!("\n - [Keyboard Demo] Demo completed! Press <Enter> to exit");
    pause();
}

fn timer_demo() {
    println!("\n ~~~~ [TIMER DEMO] ~~~~\n");
    println!("Task: Demonstrate the working of timer interrupt\n\n");
    println!(" - [Timer Demo] Press <Enter> to activate/deactivate timer...\n");
    pause();
    drivers::interrupts::toggle_timer();
    pause();
    drivers::interrupts::toggle_timer();

    println!("\n\n - [Timer Demo] Demo completed! Press <Enter> to exit\n");
    pause();
}

fn exception_demo() {
    println!("\n ~~~~ [EXCEPTION DEMO 2] ~~~~\n");
    drivers::term::print_yellow("Task: Demonstrate the working of the exception handler and double fault handler that asynchronously handle exception.\n\n");
    println!(" - [Exception Demo] Press <Enter> to raise an exception");
    pause();

    x86_64::instructions::interrupts::int3();
    println!("\n - [Exception Demo] Press <Enter> to continue");
    pause();

    drivers::term::term_cls();
    println!("\n ~~~~ [EXCEPTION DEMO 2] ~~~~\n");
    drivers::term::print_yellow("Task: Demonstrate the working of double fault handler.\n\n");
    println!(" - [Exception Demo] Press <Enter> to raise an exception");
    pause();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    println!("\n\n - [Exception Demo] Demo completed! Press <Enter> to exit");
}

fn input_demo() {
    println!("\n ~~~~ [INPUT DEMO] ~~~~\n");
    drivers::term::print_yellow(
        "Task: Demonstrate the working of input module that accepts data from user.\n\n",
    );
    print!(" - [Input Demo] Input any string: ");

    let inp = drivers::interrupts::input_from_user();
    let temp = str::from_utf8(&inp).unwrap();

    println!("\n - [Input Demo] The string you entered is: {}", temp);

    println!("\n - [Timer Demo]Demo completed! Press <Enter> to continue");
    pause();
}

fn print_welcome() {
    println!("Welcome to..\n");
    println!("     ____               _           __              ____  ");
    println!("    / __ \\_________    (_)__  _____/ /_            / __ \\ ");
    println!("   / /_/ / ___/ __ \\  / / _ \\/ ___/ __/  ______   / /_/ / ");
    println!("  / ____/ /  / /_/ / / /  __/ /__/ /_   /_____/  / _, _/  ");
    println!(" /_/   /_/   \\____/_/ /\\___/\\___/\\__/           /_/ |_|   ");
    println!("                 /___/                                   ");
    println!("\nAn operating system developed in Rust\n");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n{}", _info);

    drivers::hlt_loop()
}
