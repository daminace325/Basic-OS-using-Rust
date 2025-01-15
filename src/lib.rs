#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)] //to use x86-interrupt calling convention

use core::panic::PanicInfo;

pub mod serial; //import serial module
pub mod vga_buffer; //import module for VGA buffer
pub mod interrupts; //import interrupts module

//a new testable trait
pub trait Testable {
    fn run(&self) -> ();
}


//implement this trait for all types T Testable
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) { //a function to tests
    serial_println!("Running {} tests", tests.len()); //print to the serial interface
    for test in tests {
        test.run(); // use the new Testable trait
    }
    exit_qemu(QemuExitCode::Success); //to exit QEMU after all tests have run
}


//panic handler in test mode
pub fn test_panic_handler(info: &PanicInfo) -> ! { //exit QEMU with an error message on a panic
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
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


pub fn init() {
    interrupts::init_idt();  //call IDT from interrupt.rs
}