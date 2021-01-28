#![deny(unsafe_op_in_unsafe_fn)]
#![feature(unsafe_block_in_unsafe_fn)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(default_alloc_error_handler)]
#![feature(wake_trait)]
#![feature(async_closure)]
#![test_runner(crate::test_runner::run_all_tests)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

mod async_tasks;
mod framebuffer;
mod memory;
mod multitasking;
mod panic;
mod platform;
mod prelude;
mod serial;
mod startup;
mod test_runner;

bootloader::entry_point!(startup::main);

extern crate alloc;
