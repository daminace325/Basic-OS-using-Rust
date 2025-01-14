#![no_std] //disable standard library

use core::panic::PanicInfo;

fn main() {
}


//function called on panic
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}