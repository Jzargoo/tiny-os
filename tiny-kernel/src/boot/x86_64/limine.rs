use core::ops::Add;

use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, 
    request::{
        EntryPointRequest, FramebufferRequest, HhdmRequest, MemmapRequest, StackSizeRequest
    }
};

use x86_64::VirtAddr;

use crate::{
    hal::{    
        bios_info::BiosInfo,
        framebuffer::Framebuffer
    },
    kernel_main,
    logger::LOGGER};

#[used]
#[unsafe(link_section = ".requests_start_marker")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section=".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(65536);

#[unsafe(link_section = ".requests")]
pub static HHDM: HhdmRequest = HhdmRequest::new();

#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();

#[used]
#[unsafe(link_section=".requests")]
pub static ENTRY_REQUEST: EntryPointRequest = EntryPointRequest::new(_start);

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();
#[unsafe(no_mangle)]
// #[unsafe(link_section = ".text._start")]
pub  extern "C" fn _start() -> ! {

    LOGGER.lock().write("The kernel is starting...");

    let virt_addr = hhdm_init().expect("The kernel MUST return offset");

    let mem = virt_addr.add(0x8b000);

    let vga: *mut u8 = mem.as_mut_ptr();
    unsafe {
        vga.write(b'H');
        vga.write(0x20);
    }


    if let Some(fb) = framebuffer_init() {
        LOGGER.lock().write("The framebuffer was initilized");
        
        unsafe {
            let addr = fb.start_addr as *mut u32;

            for i in 0..(fb.width * fb.height / 10) {
                *addr.add(i as usize) = 0x00FF0000;
            }
        }
 
        let virt_addr = hhdm_init().expect("The kernel MUST return offset"); // The kernel MUST return offset
 
        let mut bi = BiosInfo::new(fb, virt_addr.as_u64());
        
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

fn hhdm_init() -> Option<VirtAddr>{
    if let Some(resp) = HHDM.response() {
        Some(
            VirtAddr::new(resp.offset)
        )
    } else {
        None
    }
}