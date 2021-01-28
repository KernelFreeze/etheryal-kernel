use bootloader::memory_region::MemoryRegion;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, Translate,
    },
    VirtAddr,
};

use self::paging::KernelFrameAllocator;

mod heap;
mod paging;

pub const HEAP_START: u64 = 0x_4444_4444_0000;
pub const HEAP_SIZE: u64 = 100 * 1024;

pub fn init_memory(memory_regions: &'static mut [MemoryRegion], physical_memory_offset: u64) {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let mut mapper = paging::init(phys_mem_offset);
    let mut frame_allocator = KernelFrameAllocator::init(memory_regions);

    init_heap(&mut mapper, &mut frame_allocator).expect("Failed to initialize kernel heap.");
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START);
        let heap_end = heap_start + HEAP_SIZE - 1u64;

        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)? }.flush()
    }

    Ok(())
}
