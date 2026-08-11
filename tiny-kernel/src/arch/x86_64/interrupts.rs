
use spin::Mutex;

use crate::arch::x86_64::interrupts::{exception_funcs::setup_idt, gdt::setup_gdt, interrupt_allocator::{InterruptVectorAllocator}};

mod exception_funcs;
mod gdt;
pub mod interrupt_allocator;

pub(in crate::arch::x86_64) fn enable_cpu_interrupts() {
    setup_gdt();
    setup_idt();
}

pub static VECTOR_INTERRUPT_ALLOCATOR: Mutex<InterruptVectorAllocator> = Mutex::new(
    InterruptVectorAllocator::default()
);
