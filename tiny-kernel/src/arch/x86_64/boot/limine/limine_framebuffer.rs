use crate::{arch::x86_64::boot::limine::limine_requests::FRAMEBUFFER_REQUEST, hal::framebuffer::Framebuffer};


pub fn framebuffer_init() -> Option<Framebuffer> {

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
