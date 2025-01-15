use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment; //using TaskStateSegment(TSS)
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor}; //using GDT(Global Descriptor Table)
use x86_64::structures::gdt::SegmentSelector;
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


    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); //since GDT is changed, reload the code segment
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); //access to TSS selector so that CPU can use it
        (gdt, Selectors { code_selector, tss_selector })
    };
}


struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() { //initialize GDT
    use x86_64::instructions::tables::load_tss; //load TSS
    use x86_64::instructions::segmentation::{CS, Segment}; //reload code segment
    
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}