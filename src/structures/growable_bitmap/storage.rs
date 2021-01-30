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

use core::ops::Range;

/// Named to clarify bit/byte operations.
const BITS_IN_BYTE: usize = 8;

/// Types implementing this trait can be used as storage for a `GrowableBitmap`.
///
/// Only fixed-size types should implement this: see the `BITS_IN_STORAGE`
/// constant requirement for this trait.
///
/// # Safety
///
/// This trait exposes several methods that are `unsafe` to call.
///
/// The given `index` must fit in `0..<Self as BitStorage>::BITS_IN_STORAGE`
/// for the behaviour of the `unsafe` methods to be correct.
pub unsafe trait BitStorage: Sized {
    /// Number of bits that can be stored in one instance of `Self`.
    ///
    /// This is a constant and types implementing this trait guarantee this
    /// will always be exact.
    const BITS_IN_STORAGE: usize = core::mem::size_of::<Self>() * BITS_IN_BYTE;

    /// Construct a new, empty instance of a `BitStorage` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let a = u8::empty();
    /// assert_eq!(a, 0);
    /// ```
    fn empty() -> Self;

    /// Returns `true` is the storage is considered empty (no `1`s anywhere).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let a = u16::empty();
    /// assert!(a.is_empty());
    ///
    /// let a = 1_u16 << 2;
    /// assert!(!a.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Gets the bit at the given index and returns `true` when it is set
    /// to 1, `false` when it is not.
    ///
    /// # Safety
    ///
    /// The given `index` must fit in `0..<Self as BitStorage>::BITS_IN_STORAGE`
    /// for the behaviour of this method to be correct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let a = u32::empty();
    /// assert!(unsafe { !a.get_bit(2) });
    ///
    /// let a = 1_u32 << 2;
    /// assert!(unsafe { a.get_bit(2) });
    /// ```
    unsafe fn get_bit(&self, index: usize) -> bool;

    /// Sets the bit at the given index and returns `true` when it is set
    /// to 1 by this call, `false` when it was already 1.
    ///
    /// # Safety
    ///
    /// The given `index` must fit in `0..<Self as BitStorage>::BITS_IN_STORAGE`
    /// for the behaviour of this method to be correct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let mut a = u64::empty();
    ///
    /// assert!(unsafe { a.set_bit(0) });
    /// assert!(unsafe { a.get_bit(0) });
    ///
    /// assert!(unsafe { a.set_bit(7) });
    /// assert!(unsafe { a.get_bit(7) });
    ///
    /// assert!(unsafe { !a.set_bit(0) });
    /// ```
    unsafe fn set_bit(&mut self, index: usize) -> bool;

    /// Clears the bit at the given index and returns `true` when it is set
    /// to 0 by this call, `false` when it was already 0.
    ///
    /// # Safety
    ///
    /// The given `index` must fit in `0..<Self as BitStorage>::BITS_IN_STORAGE`
    /// for the behaviour of this method to be correct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let mut a = u64::empty();
    ///
    /// assert!(unsafe { !a.clear_bit(56) });
    ///
    /// assert!(unsafe { a.set_bit(56) });
    /// assert!(unsafe { a.clear_bit(56) });
    /// assert!(unsafe { !a.get_bit(56) });
    /// ```
    unsafe fn clear_bit(&mut self, index: usize) -> bool;

    /// Clears the whole storage, setting `self` to the empty value.
    ///
    /// The default implementation uses `*self = Self::empty()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let mut a = u128::empty();
    ///
    /// let mut a: u128 = 42;
    /// a.clear_all();
    /// assert!(a.is_empty());
    ///
    /// let mut a: u128 = 1 << 120;
    /// a.clear_all();
    /// assert!(a.is_empty());
    /// ```
    fn clear_all(&mut self) {
        *self = Self::empty();
    }

    /// Returns the number of bits set to 1 in `Self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let a = u8::empty();
    /// assert_eq!(a.count_ones(), 0);
    ///
    /// let mut a = a;
    /// unsafe {
    ///     a.set_bit(0);
    /// }
    /// assert_eq!(a.count_ones(), 1);
    ///
    /// unsafe {
    ///     a.set_bit(3);
    /// }
    /// assert_eq!(a.count_ones(), 2);
    ///
    /// unsafe {
    ///     a.set_bit(7);
    /// }
    /// assert_eq!(a.count_ones(), 3);
    ///
    /// unsafe {
    ///     a.clear_bit(4);
    /// }
    /// assert_eq!(a.count_ones(), 3);
    ///
    /// unsafe {
    ///     a.clear_bit(7);
    /// }
    /// assert_eq!(a.count_ones(), 2);
    ///
    /// a.clear_all();
    /// assert_eq!(a.count_ones(), 0);
    /// ```
    fn count_ones(&self) -> usize;

    /// Returns the index of the first bit set in the binary representation of
    /// `self`, if any, else returns `None`.
    ///
    /// # Example
    ///
    /// Example on a u8 where the least significant bit is to the right:
    ///
    /// ```text
    /// 0b1100_0100
    ///         ^
    ///         index = 3
    /// ```
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let v = u16::empty();
    /// assert_eq!(v.first_bit_set(), None);
    ///
    /// let mut v = v;
    /// unsafe {
    ///     v.set_bit(3);
    /// }
    /// assert_eq!(v.first_bit_set(), Some(3));
    ///
    /// unsafe {
    ///     v.set_bit(7);
    /// }
    /// assert_eq!(v.first_bit_set(), Some(3));
    ///
    /// unsafe {
    ///     v.set_bit(1);
    /// }
    /// assert_eq!(v.first_bit_set(), Some(1));
    /// ```
    fn first_bit_set(&self) -> Option<usize>;

    /// Returns the index of the first empty bit in the binary representation of
    /// `self`, if any, else returns `None`.
    ///
    /// # Example
    ///
    /// Example on a u8 where the least significant bit is to the right:
    ///
    /// ```text
    /// 0b1100_0100
    ///           ^
    ///           index = 0
    /// ```
    fn first_empty_bit(&self) -> Option<usize>;

    /// Returns the index of the last bit set in the binary representation of
    /// `self`, if any, else returns `None`.
    ///
    /// # Example
    ///
    /// Example on a u8 where the least significant bit is to the right:
    ///
    /// ```text
    /// 0b0010_0011
    ///     ^
    ///     index = 5
    /// ```
    ///
    /// ```rust
    /// use growable_bitmap::BitStorage;
    ///
    /// let v = u16::empty();
    /// assert_eq!(v.last_bit_set(), None);
    ///
    /// let mut v = v;
    /// unsafe {
    ///     v.set_bit(3);
    /// }
    /// assert_eq!(v.last_bit_set(), Some(3));
    ///
    /// unsafe {
    ///     v.set_bit(1);
    /// }
    /// assert_eq!(v.last_bit_set(), Some(3));
    ///
    /// unsafe {
    ///     v.set_bit(7);
    /// }
    /// assert_eq!(v.last_bit_set(), Some(7));
    /// ```
    fn last_bit_set(&self) -> Option<usize>;

    /// Returns the index of the last empty bit in the binary representation of
    /// `self`, if any, else returns `None`.
    ///
    /// # Example
    ///
    /// Example on a u8 where the least significant bit is to the right:
    ///
    /// ```text
    /// 0b0010_0011
    ///   ^
    ///   index = 7
    /// ```
    fn last_empty_bit(&self) -> Option<usize>;

    /// Returns an iterator for the bits in the binary representation of `self`
    fn iter(&self) -> StorageIter<Self>;
}

macro_rules! bit_storage_integer_impl {
    ($int:ty, $doc:expr) => {
        #[doc = $doc]
        unsafe impl BitStorage for $int {
            #[inline(always)]
            fn empty() -> Self {
                0
            }

            #[inline(always)]
            fn is_empty(&self) -> bool {
                *self == Self::empty()
            }

            #[inline(always)]
            unsafe fn get_bit(&self, index: usize) -> bool {
                let mask = 1 << index;
                (*self & mask) != 0
            }

            #[inline(always)]
            unsafe fn set_bit(&mut self, index: usize) -> bool {
                let mask = 1 << index;
                let prev = *self & mask;

                *self |= mask;
                prev == 0
            }

            #[inline(always)]
            unsafe fn clear_bit(&mut self, index: usize) -> bool {
                let mask = 1 << index;
                let prev = *self & mask;

                *self &= !mask;
                prev != 0
            }

            #[inline(always)]
            fn count_ones(&self) -> usize {
                <$int>::count_ones(*self) as usize
            }

            fn first_bit_set(&self) -> Option<usize> {
                self.iter().position(|x| x)
            }

            fn first_empty_bit(&self) -> Option<usize> {
                self.iter().position(|x| !x)
            }

            fn last_bit_set(&self) -> Option<usize> {
                self.iter().rev().position(|x| x)
            }

            fn last_empty_bit(&self) -> Option<usize> {
                self.iter().rev().position(|x| !x)
            }

            fn iter(&self) -> StorageIter<$int> {
                StorageIter::new(self)
            }
        }
    };
    ($int:ty) => {
        bit_storage_integer_impl! {
            $int,
            concat!(
                "SAFETY: this implementation is safe because the width in ",
                "bits of an `",
                stringify!($int),
                "` is fixed and equal to `std::mem::size_of::<",
                stringify!($int),
                ">() * 8`.",
            )
        }
    };
}

bit_storage_integer_impl! { u8 }
bit_storage_integer_impl! { u16 }
bit_storage_integer_impl! { u32 }
bit_storage_integer_impl! { u64 }
bit_storage_integer_impl! { u128 }

pub struct StorageIter<'a, S: BitStorage> {
    storage: &'a S,
    range: Range<usize>,
}

impl<'a, S> StorageIter<'a, S>
where
    S: BitStorage,
{
    #[inline]
    pub fn new(storage: &'a S) -> Self {
        Self { range: (0..S::BITS_IN_STORAGE), storage }
    }

    #[inline]
    fn get(&self, bit: usize) -> bool {
        unsafe { self.storage.get_bit(bit) }
    }
}

impl<'a, S> Iterator for StorageIter<'a, S>
where
    S: BitStorage,
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

impl<'a, S> DoubleEndedIterator for StorageIter<'a, S>
where
    S: BitStorage,
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
