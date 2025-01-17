//run cargo test --test stack_overflow
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use rust_os::serial_print;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use rust_os::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rust_os::gdt::init();
    init_test_idt();

    // trigger the stack overflow
    stack_overflow();

    //if this is printed it means the stack_overflow has been handled successfully
    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)] //to silence the compiler warning that the function recurses endlessly
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() { //loads the IDT on the CPU through the load method
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler( //when double handler is called we exit QEMU with a success code making it inidcate the test has been passed
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok] Stack-Overflow handled");
    exit_qemu(QemuExitCode::Success);
    loop {}
}