// Copyright (c) 2021 Miguel Peláez
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use bootloader::BootInfo;

use crate::log::KernelLogger;
use crate::prelude::*;
use crate::tasks::executor::TaskExecutor;

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    // Initialize memory
    let memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Failed to map virtual memory address.");
    let memory_regions = &mut boot_info.memory_regions;
    crate::memory::allocator::init(memory_regions, memory_offset);

    // Pre-Initialize platform specifics
    crate::platform::pre_init();

    // Initialize device drivers
    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to adquire screen framebuffer.");
    crate::framebuffer::init(framebuffer);

    log::set_logger(&KernelLogger)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");

    info!("Starting Kernel");

    // Initialize platform specifics
    crate::platform::init();

    #[cfg(test)]
    crate::test_main();

    // Start task scheduler
    let mut task_executor = TaskExecutor::new();
    task_executor.spawn(crate::wasm::run_binary_program());
    task_executor.run();
}
