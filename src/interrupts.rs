use x86_64::structures::idt::{
    InterruptDescriptorTable,  //imported IDT(InterruptDescriptorTable) in a struct to specify a hanlder for each exception
    InterruptStackFrame   //stack to save the state of CPU just before interrupt occurs
};
use crate::println;
use lazy_static::lazy_static; //to make idt stactic since idt on its own is treated as a normal hence its time doesnt live long enough for interrupt handling


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); //creates a new IDT
        idt.breakpoint.set_handler_fn(breakpoint_handler); //add breakpoint handler into the IDT
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

//breakpoint exception test
#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}