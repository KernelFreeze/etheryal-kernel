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

use crate::prelude::*;
#[cfg(test)]
use crate::tasks::executor::TaskExecutor;

#[cfg(test)]
pub fn run_all_tests(tests: &[&dyn Testable]) {
    info!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
}

#[cfg(test)]
pub fn register_tasks(executor: &mut TaskExecutor) {
    executor.spawn(run_tests());
}

#[cfg(test)]
async fn run_tests() {
    use crate::platform::exit::{exit_with, ExitDiagnostics};

    // Make qemu log all to serial port when testing on x86-64
    #[cfg(all(test, feature = "qemu", target_arch = "x86_64"))]
    crate::logger::KERNEL_LOGGER.set_callback(logger_callback);

    crate::test_main();
    exit_with(ExitDiagnostics::Success);
}

/// Log implementation using qemu with a uart 1660 serial port
#[cfg(all(test, feature = "qemu", target_arch = "x86_64"))]
fn logger_callback() {
    use core::fmt::Write;

    use spin::{Lazy, Mutex};
    use uart_16550::SerialPort;

    static SERIAL1: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    });

    while let Some(log) = crate::logger::KERNEL_LOGGER.get_logs().pop() {
        crate::platform::interrupts::without_interrupts(|| {
            SERIAL1.lock().write_str(&log).expect("Printing to serial failed");
        });
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        info!("{}...\t", core::any::type_name::<T>());
        self();
        info!("[ok]");
    }
}
