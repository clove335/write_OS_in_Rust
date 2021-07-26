#![no_std] // We do not want to use Rust STL for making OS
#![no_main]
// Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
#![feature(custom_test_frameworks)]
#![test_runner(write_os_in_rust::test_runner)]
// apply entry point to our OS's entry point "_start"
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use write_os_in_rust::println;

#[cfg(not(test))]
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

    write_os_in_rust::init();

    fn stack_overflow() {
        stack_overflow();
    }

    // comment out below to trigger a stack overflow
    //stack_overflow();

    // comment out below to trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //};

    // comment out below to invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Did not crash!");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_os_in_rust::test_panic_handler(info)
}
