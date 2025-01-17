use x86_64::structures::idt::{
    InterruptDescriptorTable,  //imported IDT(InterruptDescriptorTable) in a struct to specify a hanlder for each exception
    InterruptStackFrame   //stack to save the state of CPU just before interrupt occurs
};
use crate::println;
use lazy_static::lazy_static; //to make idt stactic since idt on its own is treated as a normal hence its time doesnt live long enough for interrupt handling
use crate::gdt;
use pic8259::ChainedPics;
use spin;
use crate::print;

//setting offesets for PIC to the range 32-47
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); //creates a new IDT
        idt.breakpoint.set_handler_fn(breakpoint_handler); //add breakpoint handler into the IDT
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler) //double fault handler to handler exceptions who do not have a handler in IDT 
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); //to set the stack index for double fault handler in the IDT
        }
        idt[InterruptIndex::Timer.as_usize()]
           .set_handler_fn(timer_interrupt_handler); //call the timer hanlder that was causing double fault exception

        idt[InterruptIndex::Keyboard.as_usize()]
           .set_handler_fn(keyboard_interrupt_handler); //call the keyboard handler to handler interrupts from keyboard
        idt
    };
}

pub fn init_idt() { //func to initialize idt
    IDT.load();
}

//just outputs a message and pretty-prints the interrupt stack frame.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

//added a handler function for the timer interrupt that was causing double fault
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame){
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); //PIC expects an EOI(End of interrupt) else it will still be busy processing first timer interrupt
    }
}

//the Port type of the x86_64 crate to read a byte from the keyboard’s 
//data port is called a scancode and it represents the key press/release
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame){
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() }; //query to read the scancode of the pressed key
    print!("{}", scancode); //print the read scancode

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}


//breakpoint exception test
#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, //handler function for the keyboard interrupt
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}