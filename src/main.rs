#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function
#![feature(custom_test_frameworks)] //a test framework from rust on bare metal environment
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //to change the name of the generated function to call test_main()

extern crate alloc;

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use rust_os::task::{Task, simple_executor::SimpleExecutor};
use rust_os::task::keyboard;
use rust_os::task::executor::Executor;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {  //start function
    use rust_os::allocator;
    use rust_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); //execute on key presses to print keys
    executor.run();
        

    #[cfg(test)] //ensure the call only happens during tests
    test_main();

    //if this is printed that means exceptions are being handled
    println!("It did not crash!");

    rust_os::hlt_loop();
}

//function called on panic
#[cfg(not(test))] //use panic handler also on testing
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}


//panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}


//using async-await
async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}   