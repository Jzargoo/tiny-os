
use spin::Mutex;

use crate::arch::x86_64::interrupts::{interrupt_funcs::setup_idt, gdt::setup_gdt, interrupt_allocator::{InterruptVectorAllocator}};

mod interrupt_funcs;
pub mod interrupt_allocator;
pub(super)mod interrupt_stabber;

pub mod lapic;

pub(super) mod lapic_requests_options;

mod gdt;


pub(in crate::arch::x86_64) fn enable_cpu_interrupts() {
    setup_gdt();
    setup_idt();
}

pub static VECTOR_INTERRUPT_ALLOCATOR: Mutex<InterruptVectorAllocator> = Mutex::new(
    InterruptVectorAllocator::default()
);