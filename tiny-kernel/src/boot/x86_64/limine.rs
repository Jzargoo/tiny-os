use limine::{self, RequestsEndMarker, RequestsStartMarker, request::{EntryPointRequest, FramebufferRequest}};


use crate::{
    hal::{GREEN, BLACK,
    bios_info::BiosInfo,
    framebuffer::Framebuffer},
    kernel_main,
    logger::graphycal::{DISPLAY_WRITER, writer::Writer}, serial_info, serial_debug, serial_warn};

#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section=".requests")]
pub static ENTRY_REQUEST: EntryPointRequest = EntryPointRequest::new(_start);

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section=".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
// #[unsafe(link_section = ".text._start")]
pub  extern "C" fn _start() -> ! {

    serial_info!("The kernel started execution!");

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

        let writer = Writer::new(
                &fb as *const Framebuffer,
                0,
                GREEN,
                BLACK,
                8
            );
        
        DISPLAY_WRITER.init_from_ref(&writer);

        serial_debug!("The framebuffer was initilized. Its structure is {:?}", fb);        
  
        unsafe {
            let addr = fb.start_addr as *mut u32;

            for i in 0..(fb.width * fb.height / 10) {
                *addr.add(i as usize) = 0x00FF0000;
            }
        }

        let mut bi = BiosInfo::new(fb);
        
        kernel_main(&mut bi);        
    
    }else {
        serial_warn!("The framebuffer was not initilized");
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
        serial_warn!("The framebuffer was not initilized");

        None
    } 


}