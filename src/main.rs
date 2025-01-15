#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function
#![feature(custom_test_frameworks)] //a test framework from rust on bare metal environment
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //to change the name of the generated function to call test_main()

use core::panic::PanicInfo;
use rust_os::println;


#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! {  //start function
    println!("Hello World{}", "!"); //directly use println! function using macros

    rust_os::init(); 

    //invoke a page fault for double fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };


    #[cfg(test)] //ensure the call only happens during tests
    test_main();

    //if this is printed that means exceptions are being handled
    println!("It did not crash!");

    loop {}
}

//function called on panic
#[cfg(not(test))] //use panic handler also on testing
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


//panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}