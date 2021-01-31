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

use core::fmt::{self, Write};

use log::{Level, Metadata, Record};

fn print(args: fmt::Arguments) {
    #[cfg(target_arch = "x86_64")]
    crate::framebuffer::WRITER
        .lock()
        .as_mut()
        .unwrap()
        .write_fmt(args)
        .unwrap();
}

pub struct KernelLogger;

impl log::Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print(format_args!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}
