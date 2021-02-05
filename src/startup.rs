// MIT License
//
// Copyright (c) 2021 The etheryal Project Developers
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

use crate::prelude::*;
use crate::*;

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    platform::pre_init();

    // Initialize screen output
    let framebuffer = boot_info.framebuffer.as_mut();
    if let Some(framebuffer) = framebuffer {
        platform::framebuffer::init(framebuffer);
    }

    // Initialize memory allocation
    let memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Failed to map virtual memory address.");
    memory::init(&mut boot_info.memory_regions, memory_offset);

    // Initialize logger
    log::set_logger(&logger::KERNEL_LOGGER)
        .map(|_| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
    info!("etheryal kernel v{}", build_info::PKG_VERSION);
    info!("build with {}", build_info::RUSTC_VERSION);

    platform::init();
    init_scheduler();
}

fn init_scheduler() -> ! {
    use tasks::executor::TaskExecutor;

    let mut task_executor = TaskExecutor::new();

    // Setup init tasks
    #[cfg(test)]
    tests::register_tasks(&mut task_executor);
    task_executor.run();
}
