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

use bootloader::memory_region::{MemoryRegion, MemoryRegionKind};
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init(memory_regions: &'static mut [MemoryRegion], offset: u64) {
    let mut allocator = ALLOCATOR.lock();
    memory_regions.iter().filter(|r| r.kind == MemoryRegionKind::Usable).for_each(|region| unsafe {
        allocator.add_to_heap((region.start + offset) as usize, (region.end + offset) as usize);
    });
}
