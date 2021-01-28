use crate::{prelude::*, tasks::executor::TaskExecutor};
use bootloader::{boot_info::FrameBuffer, memory_region::MemoryRegion, BootInfo};

pub struct StartupInfo {
    pub memory_regions: &'static mut [MemoryRegion],
    pub memory_offset: u64,
}

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    println!("Starting Kernel");

    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to adquire screen framebuffer.");
    crate::framebuffer::init(framebuffer);

    let memory_offset = boot_info.physical_memory_offset.into_option().expect(
        "Failed to map virtual memory address. The bootloader didn't provide a memory offset",
    );
    let memory_regions = &mut boot_info.memory_regions;
    let startup_info = StartupInfo {
        memory_regions,
        memory_offset,
    };
    crate::platform::init_platform(startup_info);

    #[cfg(test)]
    crate::test_main();

    // Requires heap memory
    let mut task_executor = TaskExecutor::new();
    task_executor.run();
}
