use bootloader::memory_region::MemoryRegion;

use crate::startup::StartupInfo;

pub mod halt;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub fn init_platform(startup_info: StartupInfo) {
    x86_64::init(startup_info);
}

#[cfg(feature = "x86_32")]
pub fn init_platform() {}

#[cfg(feature = "arm64")]
pub fn init_platform() {}

#[cfg(feature = "risc_v")]
pub fn init_platform() {}
