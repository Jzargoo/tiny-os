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

use crate::{hal::{BLUE, framebuffer::{Color, Framebuffer}}, logger::graphycal::{bitmap_font::CELL_SIZE, writer::DisplayWriter}};

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
        (&mut bi.framebuffer) as *mut Framebuffer,
        0,
        Color::from_hex_rgb(0xFF3235),
        BLUE, 
        CELL_SIZE);
    
    dw.write_string("Hello World");
    
    // main();

}

