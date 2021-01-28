mod apic;
mod gdt;
mod interrupts;
mod memory;

pub fn init() {
    gdt::init();
    apic::init();
    interrupts::init_idt();
}
