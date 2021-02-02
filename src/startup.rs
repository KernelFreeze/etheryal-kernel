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

use crate::framebuffer::FramebufferWriter;
use crate::log::KernelLogger;
use crate::prelude::*;

pub fn main(boot_info: &'static mut BootInfo) -> ! {
    init_memory(boot_info);
    init_framebuffer(boot_info);
    init_logger();

    display_kernel_info();

    unsafe {
        crate::platform::pre_init();
        crate::platform::init();
    }

    #[cfg(test)]
    run_tests();

    #[cfg(not(test))]
    init_scheduler();
}

#[cfg(test)]
fn run_tests() -> ! {
    crate::test_main();
    crate::platform::permanent_halt();
}

#[cfg(not(test))]
fn init_scheduler() -> ! {
    use crate::tasks::executor::TaskExecutor;

    let mut task_executor = TaskExecutor::new();
    // task_executor.spawn(async {
    // crate::wasm::run_binary_program(&[]).await.unwrap() });
    task_executor.run();
}

#[inline(always)]
fn init_framebuffer(boot_info: &'static mut BootInfo) {
    use crate::framebuffer::init;

    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Failed to adquire screen framebuffer.");
    init(FramebufferWriter::new(framebuffer));
}

#[inline(always)]
fn init_memory(boot_info: &mut BootInfo) {
    use crate::memory::allocator::init;

    let memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Failed to map virtual memory address.");
    let memory_regions = &mut boot_info.memory_regions;
    init(memory_regions, memory_offset);
}

#[inline(always)]
fn init_logger() {
    log::set_logger(&KernelLogger)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
}

fn display_kernel_info() {
    use crate::build_info;

    if let Some(git_version) = build_info::GIT_VERSION {
        info!("etheryal kernel git-build {}", git_version);
    } else {
        info!("etheryal kernel v{}", build_info::PKG_VERSION);
    }

    info!("build with rustc {}", build_info::RUSTC_VERSION);
}
