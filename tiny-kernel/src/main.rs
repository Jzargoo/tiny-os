#![no_std]
#![no_main]

use ::core::{fmt::Write, panic::PanicInfo};
mod boot;
mod core;
mod logger;
mod allocator;
mod hal;
mod arch;

use hal::bios_info::BiosInfo;

use core::main;

use crate::{hal::{BLACK, CYAN, GREEN, WHITE, framebuffer::Framebuffer, page_allocator::PageSize}, logger::{LOGGER, graphycal::{bitmap_font::CELL_SIZE, writer::DisplayWriter}}};

pub extern crate alloc;


#[global_allocator]
pub static ALLOCATOR: allocator::SlubAllocator = allocator::SlubAllocator{}; 

#[panic_handler]
pub fn panic(qi: &PanicInfo) -> ! {
    
    print!("Kernel panic: ");
    
    println!("{}", qi);

    panic_flush!();
    
    loop {}
}

pub fn kernel_main(bi: &mut BiosInfo) {

    
    let mut dw = DisplayWriter::new(
        (&mut bi.framebuffer) as *mut Framebuffer,
        0,
        BLACK,
        GREEN, 
        CELL_SIZE);

    dw.write_string("Hello world!");
    
    
    let pages = bi.page_allocator.allocate_pages(10, PageSize::REGULAR);
    
    pages.map(|p| {
        println!("Allocated pages. Phys frame is {:?}", p);
    }).unwrap_or_else(|| {
        println!("Failed to allocate pages");
    });


    main();
}