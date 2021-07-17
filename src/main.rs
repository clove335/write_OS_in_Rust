#![no_std] // We do not want to use Rust STL for making OS
#![no_main] // Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Disable name mangling
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
