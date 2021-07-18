#![no_std] // We do not want to use Rust STL for making OS
#![no_main] // Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // Disable name mangling
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    println!("Hello World{}", "!");
    loop {}
}
