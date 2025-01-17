use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};
use x86_64::PhysAddr;
use x86_64::structures::paging::OffsetPageTable;

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read(); //read the physical frame of the active level 4 table from the CR3 register

    let phys = level_4_table_frame.start_address(); //take the physical address
    let virt = physical_memory_offset + phys.as_u64(); //get the virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr(); //convert the virtual address to raw pointer

    &mut *page_table_ptr // unsafe
}

// Initialize a new OffsetPageTable
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}