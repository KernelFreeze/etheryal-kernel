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

#[cfg(target_pointer_width = "16")]
type CInt = i16;

#[cfg(not(target_pointer_width = "16"))]
type CInt = i32;

#[cfg(target_arch = "x86_64")]
pub unsafe fn copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    super::x86_64::memory::copy_memory(dest, src, n)
}

#[cfg(not(target_arch = "x86_64"))]
pub unsafe fn copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    super::software::memory::copy_memory(dest, src, n)
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn overlapping_copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    super::x86_64::memory::overlapping_copy_memory(dest, src, n)
}

#[cfg(not(target_arch = "x86_64"))]
pub unsafe fn overlapping_copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    super::x86_64::memory::overlapping_copy_memory(dest, src, n)
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn set_bytes(dest: *mut u8, c: CInt, n: usize) {
    super::x86_64::memory::set_bytes(dest, c as u8, n)
}

#[cfg(not(target_arch = "x86_64"))]
pub unsafe fn set_bytes(dest: *mut u8, c: CInt, n: usize) -> *mut u8 {
    super::x86_64::memory::set_bytes(dest, src, n)
}
