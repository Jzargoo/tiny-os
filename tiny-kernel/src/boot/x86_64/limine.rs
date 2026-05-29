use limine::{self, RequestsEndMarker, RequestsStartMarker, request::{EntryPointRequest, FramebufferRequest, MemmapRequest, StackSizeRequest}};
use crate::hal::page_allocator;

use crate::{
    hal::{    
        bios_info::BiosInfo,
        framebuffer::Framebuffer, page_allocator::PageAllocator,
    },
    kernel_main,
    logger::LOGGER};

#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section=".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(65536);

#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();

#[used]
#[unsafe(link_section=".requests")]
pub static ENTRY_REQUEST: EntryPointRequest = EntryPointRequest::new(_start);

#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();
#[unsafe(no_mangle)]
// #[unsafe(link_section = ".text._start")]
pub  extern "C" fn _start() -> ! {

    LOGGER.lock().write("The kernel is starting...");

    if let Some(fb) = framebuffer_init() {
        LOGGER.lock().write("The framebuffer was initilized");
        
        unsafe {
            let addr = fb.start_addr as *mut u32;

            for i in 0..(fb.width * fb.height / 10) {
                *addr.add(i as usize) = 0x00FF0000;
            }
        }

        let mut bi = BiosInfo::new(fb);
        
        kernel_main(&mut bi);        
    
    } else {
        LOGGER.lock().write("The framebuffer was not initilized");
    } 


    loop {}
}

fn framebuffer_init() -> Option<Framebuffer> {

    if let Some(buff) = FRAMEBUFFER_REQUEST.response() {
        
        let buffer = buff.framebuffers()[0];
        
        let fb = Framebuffer::new(
            buffer.address(),
            buffer.width,
            buffer.height,
            buffer.pitch,
            buffer.red_mask_size, buffer.green_mask_size, buffer.blue_mask_size,
            buffer.red_mask_shift, buffer.green_mask_shift, buffer.blue_mask_shift,
            buffer.bpp / 8
        );

        Some(fb)
    
    } else {
        None
    } 


}

fn memmap_init() {
    if let Some(memmap) = MEMMAP.response() {
        LOGGER.lock().write("Memory map entry");
        let entries = memmap.entries();
        
    } else {
        LOGGER.lock().write("No memory map available");
    }
}