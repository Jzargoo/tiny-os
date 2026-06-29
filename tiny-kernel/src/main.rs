#![no_std]
#![no_main]

use ::core::{mem, panic::PanicInfo};
mod boot;
mod core;
mod logger;
mod allocator;
mod hal;
mod arch;

use alloc::{boxed::Box, vec};
use hal::bios_info::BiosInfo;

use core::main;

use crate::{allocator::SlubAllocator, hal::{BLACK, GREEN, framebuffer::Framebuffer, page_allocator::{PageAllocator, PageSize}}, logger::{LOGGER, add_sink, graphycal::{bitmap_font::CELL_SIZE, writer::DisplayWriter}}};

pub extern crate alloc;


#[global_allocator]
pub static ALLOCATOR: SlubAllocator = SlubAllocator::default(); 


#[panic_handler]
pub fn panic(qi: &PanicInfo) -> ! {
    
    print!("Kernel panic: ");
    
    #[cfg(debug_assertions)]
    println!("{}", qi);

    panic_flush!();
    
    loop {}
}

pub fn kernel_main(bi: &mut BiosInfo) {

    init_memory(bi);
    
    let dw = Box::new(DisplayWriter::new(
        (&mut bi.framebuffer) as *mut Framebuffer,
        0,
        BLACK,
        GREEN, 
        CELL_SIZE)
    );
    
    add_sink(dw);
    for _ in 0..10 {
        LOGGER.lock().flush();
    }

    main();

}

pub fn init_memory(bi: &mut BiosInfo) {

    let short_ref: &mut dyn PageAllocator = bi.page_allocator;

    let raw_dyn_ptr = short_ref as *mut dyn PageAllocator;

    unsafe {
        let static_dyn_ptr: *mut (dyn PageAllocator + 'static) = mem::transmute(raw_dyn_ptr);
        ALLOCATOR.set_page_allocator(static_dyn_ptr);
    }
}