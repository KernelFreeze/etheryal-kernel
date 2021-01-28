use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;
    println!("Kernel panic!");
    println!("{}", info);
    crate::platform::halt::halt_cpu();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::debug;

    debug!("[failed]\n");
    debug!("Error: {}\n", info);
    crate::platform::halt::halt_cpu();
}
