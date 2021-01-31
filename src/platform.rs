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

pub mod halt;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub fn init() {
    unsafe {
        x86_64::init();
    }
}

#[cfg(target_arch = "x86_64")]
pub fn pre_init() {
    x86_64::pre_init();
}

#[cfg(target_arch = "x86")]
pub fn init_platform() {}

#[cfg(target_arch = "aarch64")]
pub fn init_platform() {}

#[cfg(target_arch = "arm")]
pub fn init_platform() {}

#[cfg(target_arch = "riscv32imac")]
pub fn init_platform() {}
