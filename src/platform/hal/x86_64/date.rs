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

use chrono::prelude::*;
use cmos::{CMOSCenturyHandler, RTCDateTime, CMOS};

unsafe fn read_rtc() -> RTCDateTime {
    let mut cmos = CMOS::new();
    cmos.read_rtc(CMOSCenturyHandler::CurrentYear(
        crate::build_info::BUILT_TIME_YEAR as usize,
    ))
}

pub fn read_datetime() -> DateTime<Utc> {
    let date = unsafe { read_rtc() };
    Utc.ymd(date.year as i32, date.month as u32, date.day as u32)
        .and_hms(date.hour as u32, date.minute as u32, date.second as u32)
}
