#![no_std] // We do not want to use Rust STL for making OS
#![no_main]
// Instead of crt0(C runtime zero), which is a default entry point of Rust programs, we define a new entry point for programs
#![feature(custom_test_frameworks)]
#![test_runner(write_os_in_rust::test_runner)]
// apply entry point to our OS's entry point "_start"
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, collections::VecDeque, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use write_os_in_rust::task::{executor::Executor, keyboard, simple_executor::SimpleExecutor, Task};
use write_os_in_rust::{memory::BootInfoFrameAllocator, println};
use x86_64::structures::paging::Page;

entry_point!(kernel_main);

#[no_mangle] // Disable name mangling
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point
    // because the linker looks for default '_start' function
    use write_os_in_rust::allocator;
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

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // allocate a number (Box uses the allocate function)
    let heap_val = Box::new(41);
    println!("heap_value at {:p}", heap_val);

    // create dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create Reference Count vector
    let reference_counted_vec = Rc::new(vec![1, 2, 3]);
    let cloned_vec = reference_counted_vec.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_vec)
    );
    core::mem::drop(reference_counted_vec);
    println!("reference count is {} now", Rc::strong_count(&cloned_vec));

    // create VecDeque
    let mut deque = VecDeque::new();
    deque.push_back(4);
    deque.push_front(2);
    for i in 0..deque.len() {
        println!("index = {}, deque elem = {}", i, deque[i]);
    }

    #[cfg(test)]
    test_main();

    println!("Did not crash!");

    //learning log
    //let mut executor = SimpleExecutor::new();
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    write_os_in_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_os_in_rust::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
