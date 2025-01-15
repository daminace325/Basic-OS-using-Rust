use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment; //using TaskStateSegment(TSS)
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor}; //using GDT(Global Descriptor Table)
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; //define 0th IST entry is double fault stack

lazy_static! {  //use lazy static becoz Rust’s const evaluator is not yet powerful enough to do this initialization at compile time.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new(); //create new TSS
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; //used as stack storage

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };


    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::tss_segment(&TSS));
        gdt
    };
}

pub fn init() { //initialize GDT
    GDT.load();
}