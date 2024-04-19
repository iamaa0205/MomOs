#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(momOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use momOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    momOS::init();

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // stack_overflow();

    let ptr = 0x20479a as *mut u8;
    unsafe { let x = *ptr; }
    println!("read worked");

    // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
        
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
fn panic(info: &PanicInfo) -> ! {
    momOS::test_panic_handler(info)
}