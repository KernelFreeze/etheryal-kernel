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

use crossbeam_queue::ArrayQueue;
use log::{Log, Metadata, Record};
use spin::{Lazy, Once};

use crate::prelude::*;

const KERNEL_LOG_QUEUE_SIZE: usize = 512;

pub static KERNEL_LOGGER: KernelLogger = KernelLogger::new();
pub type LoggerCallback = fn();

pub struct KernelLogger {
    logs: Lazy<ArrayQueue<String>>,
    callback: Once<LoggerCallback>,
}

impl KernelLogger {
    pub const fn new() -> KernelLogger {
        KernelLogger {
            logs: Lazy::new(|| ArrayQueue::new(KERNEL_LOG_QUEUE_SIZE)),
            callback: Once::new(),
        }
    }

    pub fn get_logs(&self) -> &ArrayQueue<String> {
        &self.logs
    }

    pub fn set_callback(&self, callback: LoggerCallback) {
        self.callback.call_once(|| callback);
    }
}

impl Log for KernelLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // Discard last log if queue is full
        if self.logs.is_full() {
            let _ = self.logs.pop();
        }
        self.logs
            .push(format!("{} - {}\n", record.level(), record.args()))
            .expect("Failed to free log queue");
        self.flush();
    }

    fn flush(&self) {
        if let Some(callback) = self.callback.get() {
            callback();
        }
    }
}
