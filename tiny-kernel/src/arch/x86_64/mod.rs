use crate::arch::x86_64::{gdt::setup_gdt, interrupts::setup_idt};

pub mod page_allocator;
pub mod gdt;

pub mod boot;

pub mod interrupts;

fn enable_cpu_interrupts() {
    setup_gdt();
    setup_idt();
}