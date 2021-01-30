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

use bitflags::bitflags;

/// CR0, or Extended Control Register 0, is a control register which is used to
/// toggle the storing or loading of registers related to specific CPU features
/// using the XSAVE/XRSTOR instructions. It is also used with some features to
/// enable or disable the processor's ability to execute their corresponding
/// instructions.
#[derive(Debug)]
pub struct Xcr0;

bitflags! {
    pub struct Xcr0Flags: u64 {
        const FPU_MMX_STATE = 1 << 0;
        const SSE_STATE = 1 << 1;
        const AVX_STATE = 1 << 2;
        const BNDREG_STATE = 1 << 3;
        const BNDCSR_STATE = 1 << 4;
        const OPMASK_STATE = 1 << 5;
        const ZMM_HI256_STATE = 1 << 6;
        const HI16_ZMM_STATE = 1 << 7;
        const PKRU_STATE = 1 << 9;
    }
}

impl Xcr0 {
    /// Read the current set of CR0 flags.
    #[inline]
    pub fn read() -> Xcr0Flags {
        Xcr0Flags::from_bits_truncate(Self::read_raw())
    }

    /// Read the current raw CR0 value.
    #[inline]
    pub fn read_raw() -> u64 {
        use core::arch::x86_64::_xgetbv;

        unsafe { _xgetbv(0) }
    }

    /// Write CR0 flags.
    ///
    /// Preserves the value of reserved fields.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because it's possible to violate memory
    /// safety through it, e.g. by disabling paging.
    #[inline]
    pub unsafe fn write(flags: Xcr0Flags) {
        let old_value = Self::read_raw();
        let reserved = old_value & !(Xcr0Flags::all().bits());
        let new_value = reserved | flags.bits();

        unsafe {
            Self::write_raw(new_value);
        }
    }

    /// Write raw CR0 flags.
    ///
    /// Does _not_ preserve any values, including reserved fields.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because it's possible to violate memory
    /// safety through it, e.g. by disabling paging.
    #[inline]
    pub unsafe fn write_raw(value: u64) {
        use core::arch::x86_64::_xsetbv;

        unsafe {
            _xsetbv(0, value);
        }
    }

    /// Updates CR0 flags.
    ///
    /// Preserves the value of reserved fields.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because it's possible to violate memory
    /// safety through it, e.g. by disabling paging.
    #[inline]
    pub unsafe fn update<F>(f: F)
    where
        F: FnOnce(&mut Xcr0Flags), {
        let mut flags = Self::read();
        f(&mut flags);
        unsafe {
            Self::write(flags);
        }
    }
}
