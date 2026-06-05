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

use crate::{hal::{BLACK, GREEN, framebuffer::Framebuffer}, logger::graphycal::writer::DisplayWriter};

pub extern crate alloc;


#[global_allocator]
pub static ALLOCATOR: allocator::SlubAllocator = allocator::SlubAllocator{}; 

#[panic_handler]
pub fn panic(_qi: &PanicInfo) -> ! {
    panic_flush!();
    loop {}
}

pub fn kernel_main(bi: &mut BiosInfo) {
    
    let mut dw = DisplayWriter::new(
        (&bi.framebuffer) as *const Framebuffer,
        2,
        GREEN,
        BLACK, 
        8);
    
    dw.write_string("Hello World");
    
    panic!();
    
    // main();

}

