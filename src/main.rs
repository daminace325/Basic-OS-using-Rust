#![no_std] //disable standard library
#![no_main] //telling the rust compiler not to use the normal entry point chain, thus removing main function

use core::panic::PanicInfo;

#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! {
    loop {}
}

//function called on panic
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}