use crate::startup::StartupInfo;

mod apic;
mod gdt;
mod interrupts;
mod memory;

pub fn init(startup_info: StartupInfo) {
    gdt::init();
    apic::init();
    interrupts::init_idt();
    memory::init_memory(startup_info.memory_regions, startup_info.memory_offset);
}
