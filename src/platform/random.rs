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

use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;

fn u64_to_u8_array(array: [u64; 4]) -> [u8; 32] {
    unsafe { core::mem::transmute(array) }
}

/// Get a seed that is safe to use for creation of PRNGs
#[cfg(target_arch = "x86_64")]
pub fn get_secure_random() -> Option<[u8; 32]> {
    use super::hal::x86_64::rand::RdSeed;

    match RdSeed::new() {
        Some(seeder) => {
            let mut seed = [0u64; 4];

            for part in seed.iter_mut() {
                match seeder.get_u64() {
                    Some(random) => *part = random,
                    None => return None,
                }
            }

            Some(u64_to_u8_array(seed))
        },
        None => None,
    }
}

/// Get a random number generator
pub fn get_random() -> Option<ChaChaRng> {
    get_secure_random().map(ChaChaRng::from_seed)
}
