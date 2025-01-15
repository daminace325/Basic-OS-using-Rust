#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function
#![feature(custom_test_frameworks)] //a test framework from rust on bare metal environment
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] //to change the name of the generated function to call test_main()

use core::panic::PanicInfo;

mod vga_buffer; //import module for VGA buffer
mod serial; //import serial module

#[no_mangle] //disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
pub extern "C" fn _start() -> ! {  //start function
    println!("Hello World{}", "!"); //directly use println! function using macros

    #[cfg(test)] //ensure the call only happens during tests
    test_main();

    loop {}
}

//function called on panic
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) { //a function to tests
    serial_println!("Running {} tests", tests.len()); //print to the serial interface
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success); //to exit QEMU after all tests have run
}



#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(0, 1);
    serial_println!("[ok]");
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port; 

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}