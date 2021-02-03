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

macro_rules! no_mangle {
    ($(fn $fun:ident($($iid:ident : $ity:ty),+) -> $oty:ty;)+) => {
        $(
            #[no_mangle]
            pub extern "C" fn $fun($($iid: $ity),+) -> $oty {
                libm::$fun($($iid),+)
            }
        )+
    }
}

no_mangle! {
    fn acos(x: f64) -> f64;
    fn asin(x: f64) -> f64;
    fn atan(x: f64) -> f64;
    fn atan2(x: f64, y: f64) -> f64;
    fn cbrt(x: f64) -> f64;
    fn cosh(x: f64) -> f64;
    fn expm1(x: f64) -> f64;
    fn hypot(x: f64, y: f64) -> f64;
    fn log1p(x: f64) -> f64;
    fn sinh(x: f64) -> f64;
    fn tan(x: f64) -> f64;
    fn tanh(x: f64) -> f64;
    fn cos(x: f64) -> f64;
    fn cosf(x: f32) -> f32;
    fn exp(x: f64) -> f64;
    fn expf(x: f32) -> f32;
    fn log2(x: f64) -> f64;
    fn log2f(x: f32) -> f32;
    fn log10(x: f64) -> f64;
    fn log10f(x: f32) -> f32;
    fn log(x: f64) -> f64;
    fn logf(x: f32) -> f32;
    fn fmin(x: f64, y: f64) -> f64;
    fn fminf(x: f32, y: f32) -> f32;
    fn fmax(x: f64, y: f64) -> f64;
    fn fmaxf(x: f32, y: f32) -> f32;
    fn round(x: f64) -> f64;
    fn roundf(x: f32) -> f32;
    fn sin(x: f64) -> f64;
    fn sinf(x: f32) -> f32;
    fn pow(x: f64, y: f64) -> f64;
    fn powf(x: f32, y: f32) -> f32;
    fn exp2(x: f64) -> f64;
    fn exp2f(x: f32) -> f32;
    fn fmod(x: f64, y: f64) -> f64;
    fn fmodf(x: f32, y: f32) -> f32;
    fn fma(x: f64, y: f64, z: f64) -> f64;
    fn fmaf(x: f32, y: f32, z: f32) -> f32;
    fn acosf(n: f32) -> f32;
    fn asinf(n: f32) -> f32;
    fn atan2f(a: f32, b: f32) -> f32;
    fn atanf(n: f32) -> f32;
    fn cbrtf(n: f32) -> f32;
    fn coshf(n: f32) -> f32;
    fn expm1f(n: f32) -> f32;
    fn fdim(a: f64, b: f64) -> f64;
    fn fdimf(a: f32, b: f32) -> f32;
    fn hypotf(x: f32, y: f32) -> f32;
    fn log1pf(n: f32) -> f32;
    fn sinhf(n: f32) -> f32;
    fn tanf(n: f32) -> f32;
    fn tanhf(n: f32) -> f32;
    fn ldexp(f: f64, n: i32) -> f64;
    fn ldexpf(f: f32, n: i32) -> f32;
}
