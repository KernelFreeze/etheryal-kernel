#[cfg(feature = "x86_64")]
pub fn halt_cpu() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
