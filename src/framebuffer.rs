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

use core::fmt;

use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use font8x8::UnicodeFonts;
use spin::Mutex;
use volatile::Volatile;

const OFFSET: usize = 16;
const CHARACTER_SPACE: usize = 8;

pub static WRITER: Mutex<Option<FramebufferWriter>> = Mutex::new(None);

pub fn init(writer: FramebufferWriter<'static>) {
    let mut global_writer = WRITER.try_lock().unwrap();
    assert!(global_writer.is_none(), "Global writer already initialized");
    *global_writer = Some(writer);
}

pub struct FramebufferWriter<'a> {
    buffer: Volatile<&'a mut [u8]>,
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl<'a> FramebufferWriter<'a> {
    #[inline(always)]
    pub fn new(framebuffer: &'a mut FrameBuffer) -> FramebufferWriter<'a> {
        let mut writer = FramebufferWriter {
            info: framebuffer.info(),
            buffer: Volatile::new(framebuffer.buffer_mut()),
            x_pos: 0,
            y_pos: 0,
        };
        writer.clear();
        writer
    }

    fn newline(&mut self) {
        self.y_pos += OFFSET;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = OFFSET;
    }

    /// Erases all text on the screen
    pub fn clear(&mut self) {
        self.newline();
        self.buffer.fill(0);
    }

    fn shift_lines_up(&mut self) {
        let offset = self.info.stride * self.info.bytes_per_pixel * 8;
        self.buffer.copy_within(offset.., 0);
        self.y_pos -= OFFSET;
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width() {
                    self.newline();
                }
                while self.y_pos >= (self.height() - OFFSET) {
                    self.shift_lines_up();
                }
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered);
            },
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8; 8]) {
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let on = *byte & (1 << bit) != 0;
                self.write_pixel(self.x_pos + x, self.y_pos + y, on);
            }
        }
        self.x_pos += CHARACTER_SPACE;
    }

    fn write_pixel(&mut self, x: usize, y: usize, on: bool) {
        let pixel_offset = y * self.info.stride + x;
        let color = if on {
            match self.info.pixel_format {
                PixelFormat::RGB => [0xff, 0xff, 0xff, 0],
                PixelFormat::BGR => [0xff, 0xff, 0xff, 0],
                _other => [0xff, 0xff, 0xff, 0],
            }
        } else {
            [0, 0, 0, 0]
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.buffer
            .index_mut(byte_offset..(byte_offset + bytes_per_pixel))
            .copy_from_slice(&color[..bytes_per_pixel]);
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does
    /// **not** support strings with non-ASCII characters, since they can't
    /// be printed in the VGA text mode.
    fn write_string(&mut self, s: &str) {
        for char in s.chars() {
            self.write_char(char);
        }
    }
}

impl fmt::Write for FramebufferWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
