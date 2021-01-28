use self::paging::KernelFrameAllocator;
use bootloader::memory_region::MemoryRegion;
use buddy_system_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, Translate,
    },
    VirtAddr,
};

mod paging;

// kernel heap, using 100kib
pub const HEAP_START: u64 = 0x_4444_4444_0000;
pub const HEAP_SIZE: u64 = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_memory(memory_regions: &'static mut [MemoryRegion], offset: u64) {
    let offset = VirtAddr::new(offset);
    let mut mapper = paging::init(offset);
    let mut frame_allocator = KernelFrameAllocator::new(memory_regions);

    allocate(HEAP_START, HEAP_SIZE, &mut mapper, &mut frame_allocator)
        .expect("Failed to initialize kernel heap.");

    unsafe {
        ALLOCATOR
            .lock()
            .init(HEAP_START as usize, HEAP_SIZE as usize);
    }
}

pub fn allocate(
    start: u64,
    size: u64,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(start);
        let heap_end = heap_start + size - 1u64;

        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    Ok(())
}
