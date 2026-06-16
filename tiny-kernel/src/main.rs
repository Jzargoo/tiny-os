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

use crate::{hal::{BLACK, WHITE, framebuffer::Framebuffer, page_allocator::{PageAllocator, PageSize}}, logger::{LOGGER, graphycal::{bitmap_font::CELL_SIZE, writer::DisplayWriter}}};

pub extern crate alloc;


#[global_allocator]
pub static ALLOCATOR: allocator::SlubAllocator = allocator::SlubAllocator{}; 

#[panic_handler]
pub fn panic(qi: &PanicInfo) -> ! {
    
    LOGGER.lock().write("Kernel panic: ");
    
    let error_message = qi.message().as_str().unwrap_or("Unknown error");
    
    LOGGER.lock().write(error_message);

    panic_flush!();
    
    loop {}
}

pub fn kernel_main(bi: &mut BiosInfo) {
    
    let mut dw = DisplayWriter::new(
        (&mut bi.framebuffer) as *mut Framebuffer,
        0,
        BLACK,
        WHITE, 
        CELL_SIZE);

    
    
    dw.write_string("This is the looooooooooooooooooooooooooooooooooooooooooongest looooooooooooooooooooooooooooooooooooooooooong word in the woooooooooooooooooooooooooooooooooooooooooooorld");
    
    bi.page_allocator.allocate_pages(10, PageSize::REGULAR(4096));

    main();

}