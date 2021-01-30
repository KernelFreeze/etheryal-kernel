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

#![deny(unsafe_op_in_unsafe_fn)]
#![feature(unsafe_block_in_unsafe_fn)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(default_alloc_error_handler)]
#![feature(wake_trait)]
#![feature(new_uninit)]
#![feature(const_mut_refs)]
#![feature(async_closure)]
#![feature(alloc_prelude)]
#![feature(asm)]
#![feature(box_syntax)]
#![test_runner(crate::test_runner::run_all_tests)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

mod framebuffer;
mod memory;
mod panic;
mod platform;
mod prelude;
mod startup;
mod structures;
mod tasks;
mod test_runner;
mod wasm;

bootloader::entry_point!(startup::main);

extern crate alloc;
