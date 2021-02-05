// MIT License
//
// Copyright (c) 2021 Miguel Peláez
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

pub enum ExitDiagnostics {
    Success,
    Failure,
    Panic,
}

#[cfg(not(all(test, feature = "qemu")))]
pub fn exit_with(_diagnostic: ExitDiagnostics) -> ! {
    super::halt::permanent_halt();
}

#[cfg(all(test, feature = "qemu"))]
pub fn exit_with(diagnostic: ExitDiagnostics) -> ! {
    use qemu_exit::QEMUExit;

    #[cfg(target_arch = "aarch64")]
    let qemu_exit_handle = qemu_exit::AArch64::new();

    // addr: The address of sifive_test.
    #[cfg(target_arch = "riscv64")]
    let qemu_exit_handle = qemu_exit::RISCV64::new(addr);

    // io_base: I/O-base of isa-debug-exit.
    // custom_exit_success: A custom success code; Must be an odd number.
    #[cfg(target_arch = "x86_64")]
    let qemu_exit_handle = qemu_exit::X86::new(0xf4, 5);

    match diagnostic {
        ExitDiagnostics::Success => qemu_exit_handle.exit_success(),
        ExitDiagnostics::Failure => qemu_exit_handle.exit_failure(),
        ExitDiagnostics::Panic => qemu_exit_handle.exit(3),
    };
}
