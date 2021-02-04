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

use core::mem;

use crossbeam_queue::ArrayQueue;
use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use spin::Lazy;

const ENTROPY_POOL_SIZE: usize = 1024;
static ENTROPY_POOL: Lazy<ArrayQueue<u8>> = Lazy::new(create_entropy_pool);

fn create_entropy_pool() -> ArrayQueue<u8> {
    use crate::platform::datetime::get_datetime;

    let queue = ArrayQueue::new(ENTROPY_POOL_SIZE);

    let time = get_datetime().timestamp();
    let time = unsafe { mem::transmute::<i64, u64>(time) };

    // TODO: More entropy sources, a single timestamp is not safe for cryptography
    // applications
    let mut rng = ChaChaRng::seed_from_u64(time);
    let mut bytes = [0u8; ENTROPY_POOL_SIZE / 2];
    rng.fill_bytes(&mut bytes);

    for byte in bytes.iter() {
        queue.push(*byte).expect("Failed to fill entropy pool");
    }

    queue
}

fn prng_seed() -> Option<[u8; 32]> {
    if ENTROPY_POOL.len() < 32 {
        return None;
    }

    let mut seed = [0u8; 32];

    for byte in seed.iter_mut() {
        if let Some(entropy) = ENTROPY_POOL.pop() {
            *byte = entropy;
        } else {
            return None;
        }
    }

    Some(seed)
}

/// Get a seed that is safe to use for creation of PRNGs
#[cfg(target_arch = "x86_64")]
pub fn get_secure_seed() -> Option<[u8; 32]> {
    use super::hal::x86_64::rand::RdSeed;

    let seeder = RdSeed::new();

    if let Some(seeder) = seeder {
        let mut seed = [0u8; 32];

        let get_next = || {
            seeder
                .get_u32()
                .unwrap_or_else(|| ChaChaRng::from_seed(prng_seed().unwrap_or([1u8; 32])).next_u32())
                .to_le_bytes()
        };

        let mut iter = seed.as_mut().chunks_exact_mut(4);
        for chunk in &mut iter {
            chunk.copy_from_slice(&get_next());
        }
        let rem = iter.into_remainder();
        if !rem.is_empty() {
            rem.copy_from_slice(&get_next()[..rem.len()]);
        }

        Some(seed)
    } else {
        // Fallback to pseudo random if hardware generator is not available
        prng_seed()
    }
}

/// Get a seed that is safe to use for creation of PRNGs
#[cfg(not(target_arch = "x86_64"))]
pub fn get_secure_seed() -> [u8; 32] {
    prng_seed()
}

/// Get a random number generator
pub fn get_random() -> Option<ChaChaRng> {
    get_secure_seed().map(ChaChaRng::from_seed)
}

/// Try to add bytes to the kernel entropy pool
pub fn add_bytes_to_entropy_pool(bytes: &[u8]) {
    for byte in bytes {
        if ENTROPY_POOL.push(*byte).is_err() {
            return;
        }
    }
}
