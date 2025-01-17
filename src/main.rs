#![no_std] //disable Rust-standard library
#![no_main] //disable the rust compiler not to use the normal entry point chain, thus removing main function
#![feature(custom_test_frameworks)] //a test framework from rust on bare metal environment
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //to change the name of the generated function to call test_main()

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {  //start function
    use rust_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    //map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)}; //write 'New!' in white backgroud

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