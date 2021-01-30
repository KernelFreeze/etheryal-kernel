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

// The MIT License (MIT)
//
// Copyright (c) 2020 Alexis Bourget
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

mod storage;

use core::{fmt, ops::Range};

use smallvec::SmallVec;
pub use storage::BitStorage;

/// A growable compact boolean array that can be parameterized on its storage.
///
/// Bits are stored contiguously. The first value is packed into the least
/// significant bits of the first word of the backing storage.
///
/// The storage must implement the unsafe trait `BitStorage`.
///
/// # Caveats
///
/// - The `GrowableBitMap::set_bit` method may allocate way too much memory
///   compared to what you really need (if for example, you only plan to set the
///   bits between 1200 and 1400). In this case, storing the offset of 1200
///   somewhere else and storing the values in the range `0..=200` in the
///   `GrowableBitMap` is probably the most efficient solution.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GrowableBitMap<S>
where
    S: BitStorage, {
    // The storage for the bits.
    bits: SmallVec<[S; 64]>,
}

impl<S> GrowableBitMap<S>
where
    S: BitStorage + fmt::Debug + fmt::Binary + Clone,
{
    /// Creates a new, empty `GrowableBitMap`.
    ///
    /// This does not allocate anything.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// assert!(GrowableBitMap::<u8>::new().is_empty());
    /// ```
    pub fn new() -> Self {
        Self { bits: SmallVec::<[S; 64]>::from_elem(S::empty(), 1) }
    }

    /// Constructs a new, empty `GrowableBitMap` with the specified capacity
    /// **in bits**.
    ///
    /// When `capacity` is zero, nothing is allocated.
    ///
    /// When `capacity` is not zero, the bit `capacity - 1` can be set without
    /// any other allocation and the returned `GrowableBitMap` is guaranteed
    /// to be able to hold `capacity` bits without reallocating (and maybe more
    /// if the given `capacity` is not a multiple of the number of bits in one
    /// instance of the backing storage).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u16>::with_capacity(16);
    /// assert!(b.is_empty());
    /// assert_eq!(b.capacity(), 16);
    ///
    /// b.set_bit(15);
    /// assert_eq!(b.capacity(), 16);
    ///
    /// b.set_bit(17);
    /// assert!(b.capacity() >= 16);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let div = capacity / S::BITS_IN_STORAGE;
        // Ensures the allocated capacity is enough for values like 125 with a
        // storage of `u8`:
        //
        // - `div` is 15
        // - `capacity % S::BITS_IN_STORAGE` is 5 so `rem` is 1.
        //
        // The final capacity will be 16 `u8`s -> 128 bits, enough for the
        // 125 bits asked for.
        let rem = (capacity % S::BITS_IN_STORAGE != 0) as usize;

        Self { bits: SmallVec::with_capacity(div + rem) }
    }

    /// `true` if the `GrowableBitMap` is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u32>::new();
    /// assert!(b.is_empty());
    ///
    /// b.set_bit(25);
    /// assert!(!b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty() || self.bits.iter().all(|bits| bits.is_empty())
    }

    /// Gets the bit at the given index and returns `true` when it is set to 1,
    /// `false` when it is not.
    ///
    /// This will **not** panic if the index is out of range of the backing
    /// storage, only return `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u64>::new();
    /// assert!(!b.get_bit(0));
    /// assert!(!b.get_bit(15));
    ///
    /// b.set_bit(15);
    /// assert!(!b.get_bit(0));
    /// assert!(b.get_bit(15));
    /// ```
    pub fn get_bit(&self, index: usize) -> bool {
        let bits_index = index / S::BITS_IN_STORAGE;

        // Since the bits_index does not exist in the storage, the bit at
        // `index` is logically 0.
        if self.bits.len() <= bits_index {
            return false;
        }

        let elem = &self.bits[bits_index];

        // SAFETY: we have ensure throught the steps above that the index
        // passed to `elem.set_bit` is in range of `0..S::BITS_IN_STORAGE`.
        //
        // Example with a `u8`:
        //
        // `u8::BITS_IN_STORAGE` is 8.
        // `index` is 21.
        //
        // `bits_index` = 2
        // `index - bits_index * S::BITS_IN_STORAGE` = 21 - 2 * 8 = 5 < 8
        unsafe { elem.get_bit(index - bits_index * S::BITS_IN_STORAGE) }
    }

    /// Sets the bit at the given index and returns whether the bit was set
    /// to 1 by this call or not.
    ///
    /// Note: This will grow the backing storage as needed to have enough
    /// storage for the given index. If you set the bit 12800 with a storage of
    /// `u8`s the backing storage will allocate 1600 `u8`s since
    /// `sizeof::<u8>() == 1` byte.
    ///
    /// See also the `Caveats` section on `GrowableBitMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u128>::new();
    /// assert!(b.set_bit(0)); // Bit 0 was not set before, returns true.
    /// assert!(!b.set_bit(0)); // Bit 0 was already set, returns false.
    ///
    /// assert!(b.set_bit(255)); // The bitmap will grow as needed to set the bit.
    /// ```
    pub fn set_bit(&mut self, index: usize) -> bool {
        let bits_index = index / S::BITS_IN_STORAGE;

        // Ensure there are enough elements in the `bits` storage.
        if self.bits.len() <= bits_index {
            self.bits.resize_with(bits_index + 1, S::empty);
        }

        let elem = &mut self.bits[bits_index];

        // SAFETY: we have ensure throught the steps above that the index
        // passed to `elem.set_bit` is in range of `0..S::BITS_IN_STORAGE`.
        //
        // Example with a `u8`:
        //
        // `u8::BITS_IN_STORAGE` is 8.
        // `index` is 21.
        //
        // `bits_index` = 2
        // `index - bits_index * S::BITS_IN_STORAGE` = 21 - 2 * 8 = 5 < 8
        unsafe { elem.set_bit(index - bits_index * S::BITS_IN_STORAGE) }
    }

    /// Clears the bit at the given index and returns whether the bit was set
    /// to 0 by this call or not.
    ///
    /// Note: this function will never allocate nor free memory, even when
    /// the bit being cleared is the last 1 in the value at the end of the
    /// backing storage.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u8>::new();
    /// assert!(!b.clear_bit(3)); // Bit 0 was not set before, returns false.
    ///
    /// b.set_bit(3);
    /// assert!(b.clear_bit(3));
    /// ```
    ///
    /// Testing the effects on capacity:
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u8>::new();
    /// b.set_bit(125);
    ///
    /// let old_capa = b.capacity();
    /// b.clear_bit(125);
    /// assert_eq!(old_capa, b.capacity());
    /// ```
    pub fn clear_bit(&mut self, index: usize) -> bool {
        let bits_index = index / S::BITS_IN_STORAGE;

        // Since the bits_index does not exist in the storage, the bit at
        // `index` is logically 0.
        if self.bits.len() <= bits_index {
            return false;
        }

        let elem = &mut self.bits[bits_index];

        // SAFETY: we have ensure throught the steps above that the index
        // passed to `elem.set_bit` is in range of `0..S::BITS_IN_STORAGE`.
        //
        // Example with a `u8`:
        //
        // `u8::BITS_IN_STORAGE` is 8.
        // `index` is 21.
        //
        // `bits_index` = 2
        // `index - bits_index * S::BITS_IN_STORAGE` = 21 - 2 * 8 = 5 < 8
        unsafe { elem.clear_bit(index - bits_index * S::BITS_IN_STORAGE) }
    }

    /// Clears the bitmap, removing all values (setting them all to 0).
    ///
    /// This method has no effect on the allocated capacity of the bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u16>::new();
    /// b.set_bit(4);
    /// assert!(!b.is_empty());
    ///
    /// b.clear();
    /// assert!(b.is_empty());
    /// ```
    ///
    /// Testing the effects on capacity:
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u16>::new();
    /// b.set_bit(125);
    ///
    /// let old_capa = b.capacity();
    /// b.clear();
    /// assert_eq!(old_capa, b.capacity());
    /// ```
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// Counts the number of bits that are set to 1 in the whole bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u32>::new();
    /// assert_eq!(b.count_ones(), 0);
    ///
    /// b.set_bit(2);
    /// assert_eq!(b.count_ones(), 1);
    ///
    /// b.set_bit(9);
    /// assert_eq!(b.count_ones(), 2);
    ///
    /// b.clear();
    /// assert_eq!(b.count_ones(), 0);
    /// ```
    pub fn count_ones(&self) -> usize {
        self.bits.iter().map(|store| store.count_ones() as usize).sum::<usize>()
    }

    /// Returns the number of bits the bitmap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u64>::new();
    /// assert_eq!(b.capacity(), 0);
    ///
    /// b.set_bit(380);
    /// assert_eq!(b.capacity(), 384);
    /// ```
    pub fn capacity(&self) -> usize {
        self.bits.capacity() * S::BITS_IN_STORAGE
    }

    /// Shrinks the capacity of the `GrowableBitMap` as much as possible.
    ///
    /// It will drop down as close as possible to the length needed to store
    /// the last bit set to 1 and not more but the allocator may still inform
    /// the bitmap that there is space for a few more elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::{BitStorage, GrowableBitMap};
    ///
    /// let mut b = GrowableBitMap::<u128>::with_capacity(381);
    ///
    /// b.set_bit(127);
    /// b.set_bit(380);
    /// assert_eq!(b.capacity(), 384);
    ///
    /// b.clear_bit(380);
    /// b.shrink_to_fit();
    /// assert_eq!(b.capacity(), 128);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        // Ignoring the values at the end that are 0.
        let last_set_bit_index = self.bits.iter().rev().skip_while(|&store| store.is_empty()).count();

        self.bits.truncate(last_set_bit_index);
        self.bits.shrink_to_fit();
    }

    /// Returns an iterator for the bits in the binary representation of `self`
    pub fn iter(&self) -> GrowableBitMapIter<S> {
        GrowableBitMapIter::new(self)
    }

    pub fn first_empty_bit(&self) -> Option<usize> {
        self.iter().position(|byte| !byte)
    }
}

pub struct GrowableBitMapIter<'a, S: BitStorage> {
    storage: &'a GrowableBitMap<S>,
    range: Range<usize>,
}

impl<'a, S> GrowableBitMapIter<'a, S>
where
    S: BitStorage + fmt::Debug + fmt::Binary + Clone,
{
    #[inline]
    pub fn new(storage: &'a GrowableBitMap<S>) -> Self {
        Self { range: (0..storage.capacity()), storage }
    }

    #[inline]
    fn get(&self, bit: usize) -> bool {
        self.storage.get_bit(bit)
    }
}

impl<'a, S> Iterator for GrowableBitMapIter<'a, S>
where
    S: BitStorage + fmt::Debug + fmt::Binary + Clone,
{
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.range.next().map(|x| self.get(x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<bool> {
        self.range.nth(n).map(|x| self.get(x))
    }

    #[inline]
    fn last(mut self) -> Option<bool> {
        self.next_back()
    }

    #[inline]
    fn min(mut self) -> Option<bool> {
        self.next()
    }

    #[inline]
    fn max(mut self) -> Option<bool> {
        self.next_back()
    }
}

impl<'a, S> DoubleEndedIterator for GrowableBitMapIter<'a, S>
where
    S: BitStorage + fmt::Debug + fmt::Binary + Clone,
{
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        self.range.next_back().map(|x| self.get(x))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<bool> {
        self.range.nth_back(n).map(|x| self.get(x))
    }
}

impl<S> fmt::Debug for GrowableBitMap<S>
where
    S: BitStorage + fmt::Debug + fmt::Binary + Clone,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[")?;

        if self.bits.len() > 1 {
            for bit in &self.bits[0..self.bits.len() - 1] {
                write!(fmt, "{:#b}, ", bit)?;
            }
        }
        if self.bits.len() > 0 {
            write!(fmt, "{:#b}", &self.bits[self.bits.len() - 1])?;
        }
        write!(fmt, "]")?;
        Ok(())
    }
}
