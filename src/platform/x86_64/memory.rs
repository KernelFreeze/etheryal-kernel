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

#[inline(always)]
pub unsafe extern "C" fn copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    copy_forward(dest, src, n);
    dest
}

#[inline(always)]
pub unsafe fn overlapping_copy_memory(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let delta = (dest as usize).wrapping_sub(src as usize);
    if delta >= n {
        // We can copy forwards because either dest is far enough ahead of src,
        // or src is ahead of dest (and delta overflowed).
        copy_forward(dest, src, n);
    } else {
        copy_backward(dest, src, n);
    }
    dest
}

#[inline(always)]
pub unsafe fn set_bytes(dest: *mut u8, c: u8, count: usize) {
    // FIXME: Use the Intel syntax
    asm!(
        "repe stosb %al, (%rdi)",
        inout("rcx") count => _,
        inout("rdi") dest => _,
        inout("al") c => _,
        options(att_syntax, nostack, preserves_flags)
    )
}

#[inline(always)]
unsafe fn copy_forward(dest: *mut u8, src: *const u8, count: usize) {
    // FIXME: Use the Intel syntax
    asm!(
        "repe movsb (%rsi), (%rdi)",
        inout("rcx") count => _,
        inout("rdi") dest => _,
        inout("rsi") src => _,
        options(att_syntax, nostack, preserves_flags)
    );
}

#[inline(always)]
unsafe fn copy_backward(dest: *mut u8, src: *const u8, count: usize) {
    let qword_count = count >> 3;
    let byte_count = count & 0b111;
    // FIXME: Use the Intel syntax
    asm!(
        "std",
        "repe movsq (%rsi), (%rdi)",
        "movl {byte_count:e}, %ecx",
        "addq $7, %rdi",
        "addq $7, %rsi",
        "repe movsb (%rsi), (%rdi)",
        "cld",
        byte_count = in(reg) byte_count,
        inout("rcx") qword_count => _,
        inout("rdi") dest.add(count).wrapping_sub(8) => _,
        inout("rsi") src.add(count).wrapping_sub(8) => _,
        options(att_syntax, nostack)
    );
}
