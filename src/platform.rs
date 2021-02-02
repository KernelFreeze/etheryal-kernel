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

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn init() {
    x86_64::init();
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn pre_init() {
    x86_64::pre_init();
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub fn init_platform() {}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn init_platform() {}

#[cfg(target_arch = "arm")]
#[inline(always)]
pub fn init_platform() {}

#[cfg(target_arch = "riscv32imac")]
#[inline(always)]
pub fn init_platform() {}

pub fn permanent_halt() -> ! {
    #[cfg(target_arch = "x86_64")]
    self::halt::permanent_halt()
}

pub fn temporal_halt() {
    #[cfg(target_arch = "x86_64")]
    self::halt::temporal_halt()
}

mod halt;
mod software;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(target_arch = "riscv32imac")]
mod riscv32imac;

#[cfg(target_arch = "x86_64")]
mod x86_64;
