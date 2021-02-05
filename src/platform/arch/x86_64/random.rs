// MIT License
//
// Copyright (c) 2021 Miguel Peláez
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

#[derive(Copy, Clone, Debug)]
/// Used to obtain random numbers using x86_64's RDSEED opcode
struct RdSeed(());

impl RdSeed {
    /// Creates Some(RdSeed) if RDSEED is supported, None otherwise
    #[inline]
    pub fn new() -> Option<Self> {
        if core_detect::is_x86_feature_detected!("rdseed") {
            Some(RdSeed(()))
        } else {
            None
        }
    }

    /// Uniformly sampled u64.
    /// May fail in rare circumstances or heavy load.
    #[inline]
    pub fn get_u64(self) -> Option<u64> {
        let mut res: u64 = 0;
        unsafe {
            match core::arch::x86_64::_rdseed64_step(&mut res) {
                1 => Some(res),
                x => {
                    debug_assert_eq!(x, 0, "RdSeed64 returned non-binary value");
                    None
                },
            }
        }
    }

    /// Uniformly sampled u32.
    /// May fail in rare circumstances or heavy load.
    #[inline]
    pub fn get_u32(self) -> Option<u32> {
        let mut res: u32 = 0;
        unsafe {
            match core::arch::x86_64::_rdseed32_step(&mut res) {
                1 => Some(res),
                x => {
                    debug_assert_eq!(x, 0, "RdSeed32 returned non-binary value");
                    None
                },
            }
        }
    }

    /// Uniformly sampled u16.
    /// May fail in rare circumstances or heavy load.
    #[inline]
    pub fn get_u16(self) -> Option<u16> {
        let mut res: u16 = 0;
        unsafe {
            match core::arch::x86_64::_rdseed16_step(&mut res) {
                1 => Some(res),
                x => {
                    debug_assert_eq!(x, 0, "RdSeed16 returned non-binary value");
                    None
                },
            }
        }
    }
}

pub async fn get_secure_random() -> Option<[u8; 32]> {
    fn u64_to_u8_array(array: [u64; 4]) -> [u8; 32] {
        unsafe { core::mem::transmute(array) }
    }

    match RdSeed::new() {
        Some(seeder) => {
            let mut seed = [0u64; 4];

            for part in seed.iter_mut() {
                match seeder.get_u64() {
                    Some(random) => *part = random,
                    None => return None,
                }
                crate::tasks::park::yield_now().await;
            }

            Some(u64_to_u8_array(seed))
        },
        None => None,
    }
}
