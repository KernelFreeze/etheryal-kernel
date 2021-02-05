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

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::platform;
use crate::platform::exit::{exit_with, ExitDiagnostics};
use crate::prelude::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(not(test))]
    error!("Kernel panic!");

    #[cfg(test)]
    error!("Kernel panic!");
    info!("{}", info);

    // Clear screen and display error message
    platform::interrupts::without_interrupts(|| {
        if let Some(framebuffer) = platform::framebuffer::WRITER.lock().as_mut() {
            framebuffer.clear();

            let _ = framebuffer.write_str("You need to restart your computer.\n");
            let _ = framebuffer.write_str("Press the restart button, or hold the power button.\n\n");
            let _ = framebuffer.write_str("etheryal kernel encountered an error.\n");
            let _ = framebuffer.write_fmt(format_args!("{}\n", info));
        }
    });

    exit_with(ExitDiagnostics::Panic);
}
