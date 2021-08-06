#![no_std] // We do not want to use Rust STL for making OS
#![no_main]
// Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
#![feature(custom_test_frameworks)]
#![test_runner(write_os_in_rust::test_runner)]
// apply entry point to our OS's entry point "_start"
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// TODO: delete after commit, code below was left for learning logs
//use write_os_in_rust::{memory::translate_addr, println};
use write_os_in_rust::{memory::BootInfoFrameAllocator, println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    write_os_in_rust::hlt_loop();
}

use bootloader::{entry_point, BootInfo};
// TODO: delete after commit, code below was left for learning logs
// use x86_64::structures::paging::{Page, PageTable, Translate};
use x86_64::structures::paging::Page;

entry_point!(kernel_main);

#[no_mangle] // Disable name mangling
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    use write_os_in_rust::memory;
    use x86_64::VirtAddr;
    // TODO: delete after commit, code below was left for learning logs
    // use write_os_in_rust::memory::active_level_4_table;

    println!("Hello World{}", "!");
    write_os_in_rust::init();

    fn stack_overflow() {
        stack_overflow();
    }

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // TODO: delete after commit, code below was left for learning logs
    // let l4_table = unsafe { active_level_4_table(physical_memory_offset) };
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    // TODO: delete after commit, code below was left for learning logs
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write string to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // TODO: delete after commit, code below was left for learning logs
    // move to memory.rs,
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
    // TODO: delete after commit, code below was left for learning logs
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // TODO: delete after commit, code below was left for learning logs
    //     //let phys = unsafe { translate_addr(virt, physical_memory_offset) };
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // comment out below to trigger a stack overflow
    //stack_overflow();

    // comment out below to trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //};

    // comment out below to invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    //let ptr = 0xdeadbeef as *mut u32;
    // set this address from the instruction_pointer printed as debug messages
    //let mut ptr = 0x204c4b as *mut u32;
    //unsafe {
    //    let _x = *ptr;
    //}
    //println!("Read worked!");
    // set this address from the instruction_pointer printed as debug messages
    //ptr = 0x204d4a as *mut u32;
    //unsafe {
    //    *ptr = 42;
    //}

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
