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
use rand_core::{RngCore, SeedableRng};

fn prng_seed() -> [u8; 32] {
    let seed = [0u8; 32];
    let mut rng = ChaChaRng::from_seed(seed);

    let mut results = [0u8; 32];
    rng.fill_bytes(&mut results);
    results
}

#[cfg(target_arch = "x86_64")]
fn x86_64_seed() -> [u8; 32] {
    use super::hal::x86_64::rand::RdSeed;

    fn u32_to_bytes(x: u32) -> [u8; 4] {
        let b1: u8 = ((x >> 24) & 0xff) as u8;
        let b2: u8 = ((x >> 16) & 0xff) as u8;
        let b3: u8 = ((x >> 8) & 0xff) as u8;
        let b4: u8 = (x & 0xff) as u8;

        return [b1, b2, b3, b4];
    }

    let seeder = RdSeed::new();

    if let Some(seeder) = seeder {
        let mut seed = [0u8; 32];

        let get_next = || {
            let bytes = seeder
                .get_u32()
                .unwrap_or_else(|| ChaChaRng::from_seed(prng_seed()).next_u32());
            u32_to_bytes(bytes)
        };

        let mut iter = seed.as_mut().chunks_exact_mut(4);
        for chunk in &mut iter {
            chunk.copy_from_slice(&get_next());
        }
        let rem = iter.into_remainder();
        if !rem.is_empty() {
            rem.copy_from_slice(&get_next()[..rem.len()]);
        }
        seed
    } else {
        // Fallback to pseudo random if hardware generator is not available
        prng_seed()
    }
}

#[cfg(target_arch = "x86_64")]
pub fn get_secure_seed() -> [u8; 32] {
    x86_64_seed()
}

#[cfg(not(target_arch = "x86_64"))]
pub fn get_secure_seed() -> [u8; 32] {
    prng_seed()
}

pub fn get_random() -> ChaChaRng {
    ChaChaRng::from_seed(get_secure_seed())
}
