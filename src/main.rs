#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function

use core::panic::PanicInfo;

mod vga_buffer; //import module for VGA buffer

#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! {  //start function
    use core::fmt::Write; //directly running the print function
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

//function called on panic
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}