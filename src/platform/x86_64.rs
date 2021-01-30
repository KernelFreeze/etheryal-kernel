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

use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

use self::registers::control::{Xcr0, Xcr0Flags};

mod apic;
mod gdt;
mod interrupts;
mod registers;

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
