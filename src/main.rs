#![no_std] // We do not want to use Rust STL for making OS
#![no_main] // Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
#[no_mangle] // Disable name mangling
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
