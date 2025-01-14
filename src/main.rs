#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function

use core::panic::PanicInfo;

mod vga_buffer; //import module for VGA buffer

#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! {  //start function
    vga_buffer::print_something(); //calling print_something() function from vga_buffer.rs

    loop {}
}

//function called on panic
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}