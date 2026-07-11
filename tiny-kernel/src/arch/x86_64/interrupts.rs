
use crate::arch::x86_64::interrupts::{exception_funcs::setup_idt, gdt::setup_gdt};

mod exception_funcs;
pub mod interrupt_structures;
pub mod interrupts_funcs;
mod gdt;

pub(in crate::arch::x86_64) fn enable_cpu_interrupts() {
    setup_gdt();
    setup_idt();
}
