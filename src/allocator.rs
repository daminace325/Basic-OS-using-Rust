#[global_allocator] //tells the Rust compiler which allocator instance it should use as the global heap allocator
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use linked_list_allocator::LockedHeap;

pub struct Dummy;
pub const HEAP_START: usize = 0x_4444_4444_0000; //memory starting address
pub const HEAP_SIZE: usize = 100 * 1024; //set heap size to 100 KiB


unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut() //alloc always returns a null pointer
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called") //panic in dealloc bcoz alloc never returns any memory
    }
}



pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>, //takes mutable mapper reference limited to 4KiB
    frame_allocator: &mut impl FrameAllocator<Size4KiB>, //takes mutable frameAllocator reference(4KiB)
) -> Result<(), MapToError<Size4KiB>> {
    //create a range of page that we want to map
    let page_range = { 
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        //convert the heap addresses into page types
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page) //create a page range
    };

    //map all pages of the page range created above
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    //initialize the allocator after creating the heap
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}