#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(xaoc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::println;
use crate::serial_print;
use crate::serial_println;
use crate::print;
use crate::tests_main;
use crate::panic_wait;
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::PageTable;
use xaoc_os::memory;
use x86_64::structures::paging::Page;
use xaoc_os::memory::BootInfoFrameAllocator;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use xaoc_os::task::{Task, simple_executor::SimpleExecutor, executor::Executor};
use xaoc_os::task::keyboard;

//nterpart to this file.
//global_asm!(include_str!("boot.s"));
// #[cfg(test)]
// fn test_main() {

// }

extern crate alloc;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {

    use x86_64::VirtAddr;
    use xaoc_os::allocator;

    println!("Hello World{}", "!");
    xaoc_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
    // as before
    #[cfg(test)]
    tests_main();

    println!("It did not crash!");
    xaoc_os::hlt_loop();
}


// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Hello World{}", "!");

//     #[cfg(test)]
//     tests_main();

//     xaoc_os::init();
//     // fn stack_overflow() {
//     //     stack_overflow(); // for each recursion, the return address is pushed
//     // }

//     let ptr = 0x2068fa as *mut u8;
//     unsafe { let x = *ptr;}
//     println!("read worked");

//     // unsafe {  *ptr = 42;}
//     // println!("write worked");
//     use x86_64::registers::control::Cr3;

//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

//     // // trigger a stack overflow
//     // stack_overflow();
//     println!("It did not crash!");
//     xaoc_os::hlt_loop();
//     // loop {
//     //     use xaoc_os::print;
//     //     print!("-");
//     // }

// }


#[no_mangle]
pub unsafe  fn _start_rust() -> ! {

    crate::kernel_init();

}

#[test_case]
fn trivial_assertion1() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok1114]");
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok222]");
}












// pub trait Testable {
//     fn run(&self) -> ();
// }

// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
