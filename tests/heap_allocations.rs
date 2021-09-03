#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(write_os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use write_os_in_rust::allocator;
    use write_os_in_rust::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    write_os_in_rust::init();
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_os_in_rust::test_panic_handler(info)
}

use alloc::boxed::Box;

// allocate a number (Box uses the allocate function)
#[test_case]
fn simple_allocation() {
    let heap_val_1 = Box::new(41);
    let heap_val_2 = Box::new(13);
    assert_eq!(*heap_val_1, 41);
    assert_eq!(*heap_val_2, 13);
}

use alloc::vec::Vec;

// create dynamically sized vector
#[test_case]
fn large_vec() {
    let n = 2000;
    let mut vec = Vec::new();
    for i in 0..2000 {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

use write_os_in_rust::allocator::HEAP_SIZE;

// check Box to see if pointer alloc works
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

// check bump allocations does not work when variable that lives long during the whole loop exists
#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}

use alloc::collections::VecDeque;

// see if deque is properly allocated and changed
#[test_case]
fn deque_operation() {
    let mut deque = VecDeque::new();
    deque.push_back(4);
    deque.push_front(2);
    assert_eq!(deque[0], 2);
    assert_eq!(deque[1], 4);
    deque.push_back(6);
    deque.pop_front();
    assert_eq!(deque[1], 6);
    deque.pop_back();
    deque.pop_back();
    assert_eq!(deque.is_empty(), true);
}
