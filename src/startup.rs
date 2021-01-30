// Copyright 2021 Miguel Peláez <kernelfreeze@outlook.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bootloader::BootInfo;

use crate::{prelude::*, tasks::executor::TaskExecutor};

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    // Initialize memory
    let memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Failed to map virtual memory address. The bootloader didn't provide a memory offset");
    let memory_regions = &mut boot_info.memory_regions;
    crate::memory::allocator::init(memory_regions, memory_offset);

    // Pre-Initialize platform specifics
    crate::platform::pre_init_platform();

    // Initialize device drivers
    let framebuffer = boot_info.framebuffer.as_mut().expect("Failed to adquire screen framebuffer.");
    crate::framebuffer::init(framebuffer);

    println!("Starting Kernel");

    // Initialize platform specifics
    crate::platform::init_platform();

    #[cfg(test)]
    crate::test_main();

    // Start task scheduler
    let mut task_executor = TaskExecutor::new();
    task_executor.run();
}
