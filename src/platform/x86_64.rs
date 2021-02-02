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

use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

use self::registers::control::{Xcr0, Xcr0Flags};

pub(super) mod apic;
pub(super) mod gdt;
pub(super) mod interrupts;
pub(super) mod memory;
pub(super) mod registers;

/// Enable SIMD
///
/// This function is unsafe because sets CPU registers
unsafe fn enable_simd() {
    // Enable co-processor (FPU)
    Cr0::update(|x| *x |= Cr0Flags::MONITOR_COPROCESSOR);

    // Enable SSE
    Cr4::update(|x| *x |= Cr4Flags::OSFXSR | Cr4Flags::OSXMMEXCPT_ENABLE | Cr4Flags::OSXSAVE);

    // Enable AVX if available
    if core_detect::is_x86_feature_detected!("avx") {
        Xcr0::update(|x| *x |= Xcr0Flags::SSE_STATE | Xcr0Flags::AVX_STATE);
    }
}

pub fn pre_init() {}

pub unsafe fn init() {
    // Initialize Global Descriptor Table
    gdt::init();

    // Initialize Interrupt descriptor table
    interrupts::init_idt();

    // Enable SSE and AVX
    enable_simd();

    apic::init();
}
