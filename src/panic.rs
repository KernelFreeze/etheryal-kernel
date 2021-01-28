use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic!");
    println!("{}", info);
    crate::platform::halt::halt_cpu();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    crate::platform::halt::halt_cpu();
}
