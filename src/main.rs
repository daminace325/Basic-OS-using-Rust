#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!"; //string Hello World in a variable

#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! { //function written to print Hello World
    let vga_buffer = 0xb8000 as *mut u8; //cast integer into a raw pointer

    for (i, &byte) in HELLO.iter().enumerate() { //iterate over the bytes of static HELLO byte string
        unsafe {  //using unsafe beacuse of raw pointer as they can point anywhere
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //color 0xb(light cyan)
        }
    }

    loop {}
}

//function called on panic
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}