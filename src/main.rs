#![no_std]  //disable the automatic inclusion of the standard library
#![no_main]  //o tell the Rust compiler that we don’t want to use the normal entry point chain of C environment
#![feature(custom_test_frameworks)]
#![test_runner(momOS::test_runner)]
#![reexport_test_harness_main = "test_main"] //The custom test frameworks feature generates a main function that calls test_runner, but this function is ignored because we use the #[no_main] attribute and provide our own entry point.To fix this, we first need to change the name of the generated function to something different than main through the reexport_test_harness_main attribute.
#![feature(const_mut_refs)]

use core::panic::PanicInfo;
use momOS::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use momOS::task::{Task, simple_executor::SimpleExecutor};
use momOS::task::keyboard;
use momOS::task::executor::Executor; 
extern crate alloc;
//use alloc::boxed::Box;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use momOS::memory;
    use momOS::allocator;
    // use x86_64::{structures::paging::Translate, VirtAddr};
    use x86_64::{structures::paging::Page, VirtAddr}; 
    use momOS::memory::BootInfoFrameAllocator;
    println!("Hello World{}", "!");
    momOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =  unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    // write the string `New!` to the screen through the new mapping
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();
    println!("It did not crash!");

    momOS::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    momOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
//The PanicInfo parameter contains the file and line where the panic happened and the optional panic message.
fn panic(info: &PanicInfo) -> ! {
    momOS::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


//json file is created to solve target problem by writing our own target
