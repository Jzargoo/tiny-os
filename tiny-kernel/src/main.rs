#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use ::core::{mem, panic::PanicInfo};
mod core;
mod acpi;
mod logger;
mod allocator;
mod hal;
mod arch;

use alloc::{boxed::Box};
use hal::bios_info::BiosInfo;

use core::main;

use crate::{allocator::SlubAllocator, arch::{x86_64::boot::limine::hlt_loop}, hal::{BLACK, GREEN, framebuffer::Framebuffer, page_allocator::PageAllocator}, logger::{graphycal::{bitmap_font::CELL_SIZE, writer::DisplayWriter}}};

pub extern crate alloc;



#[global_allocator]
pub static ALLOCATOR: SlubAllocator = SlubAllocator::default(); 


#[panic_handler]
pub fn panic(qi: &PanicInfo) -> ! {
    
    print!("Kernel panic: ");
    
    #[cfg(debug_assertions)]
    println!("{}", qi);

    panic_flush!();
    
    hlt_loop()
}

pub fn kernel_main(bi: &mut BiosInfo) {

    init_memory(bi);
    
    let mut dw = Box::new(DisplayWriter::new(
        (&mut bi.framebuffer) as *mut Framebuffer,
        0,
        BLACK,
        GREEN, 
        CELL_SIZE)
    );

    dw.write_string("string\n");


    dw.write_string("string again");    

    main();
   
    panic!("Test panic!");
    
}

pub fn init_memory(bi: &mut BiosInfo) {

    let short_ref: &mut dyn PageAllocator = bi.page_allocator;

    let raw_dyn_ptr = short_ref as *mut dyn PageAllocator;

    unsafe {
        let static_dyn_ptr: *mut (dyn PageAllocator + 'static) = mem::transmute(raw_dyn_ptr);
        ALLOCATOR.set_page_allocator(static_dyn_ptr);
    }
}