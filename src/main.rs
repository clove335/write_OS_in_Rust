#![no_std] // We do not want to use Rust STL for making OS
#![no_main]
// Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
#![feature(custom_test_frameworks)]
#![test_runner(write_os_in_rust::test_runner)]
// apply entry point to our OS's entry point "_start"
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use write_os_in_rust::{memory::BootInfoFrameAllocator, println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    write_os_in_rust::hlt_loop();
}

use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::Page;

entry_point!(kernel_main);

#[no_mangle] // Disable name mangling
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    use write_os_in_rust::memory;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    write_os_in_rust::init();

    fn stack_overflow() {
        stack_overflow();
    }

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write string to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("Did not crash!");
    write_os_in_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_os_in_rust::test_panic_handler(info)
}
