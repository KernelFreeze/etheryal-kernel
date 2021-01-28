pub mod halt;
pub mod memory;

#[cfg(feature = "x86_64")]
mod x86_64;

#[cfg(feature = "x86_64")]
pub fn init_platform() {
    x86_64::init();
}

#[cfg(feature = "x86_32")]
pub fn init_platform() {}

#[cfg(feature = "arm64")]
pub fn init_platform() {}

#[cfg(feature = "risc_v")]
pub fn init_platform() {}
