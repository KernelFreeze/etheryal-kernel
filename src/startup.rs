use crate::{prelude::*, async_tasks::executor::TaskExecutor};
use bootloader::BootInfo;

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    debug!("Starting Kernel");

    let frame_buffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to adquire screen framebuffer.");
    crate::framebuffer::init(frame_buffer);
    crate::platform::init_platform();

    let memory_offset = boot_info.physical_memory_offset.into_option().expect(
        "Failed to map virtual memory address. The bootloader didn't provide a memory offset",
    );
    crate::memory::init_memory(&mut boot_info.memory_regions, memory_offset);

    #[cfg(test)]
    crate::test_main();

    let mut task_executor = TaskExecutor::new();
    task_executor.run();
}
