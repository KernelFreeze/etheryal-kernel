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

// Copyright 2019-2020 Jiajie Chen
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
//

use core::{
    alloc::Layout,
    cmp::{max, min},
    fmt,
    mem::size_of,
    ptr::NonNull,
};

use smallvec::SmallVec;

use super::growable_bitmap::GrowableBitMap;

pub struct Heap {
    free_list: GrowableBitMap<u8>,
}

impl Heap {
    /// Create an empty heap
    pub fn new() -> Self {
        Heap { free_list: GrowableBitMap::new() }
    }

    /// Add a range of memory [start, end) to the heap
    pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {}

    /// Alloc a range of memory from the heap satifying `layout` requirements
    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(layout.size().next_power_of_two(), max(layout.align(), size_of::<usize>()));
        let class = size.trailing_zeros() as usize;

        Err(())
    }

    /// Dealloc a range of memory from the heap
    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(layout.size().next_power_of_two(), max(layout.align(), size_of::<usize>()));
        let class = size.trailing_zeros() as usize;
    }
}
