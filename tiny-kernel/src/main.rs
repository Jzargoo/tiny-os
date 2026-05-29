#![no_std]
#![no_main]

use ::core::panic::PanicInfo;
mod boot;
mod core;
mod logger;
mod allocator;
mod hal;

use hal::bios_info::BiosInfo;

use core::main;

pub extern crate alloc;


#[global_allocator]
pub static ALLOCATOR: allocator::HeapAllocatr = allocator::HeapAllocatr{}; 

#[panic_handler]
pub fn panic(_qi: &PanicInfo) -> ! {
    panic_flush!();
    loop {}
}

pub fn kernel_main(_bi: &mut BiosInfo) {
    main();
}

