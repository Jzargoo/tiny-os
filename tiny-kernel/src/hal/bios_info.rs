use crate::hal::framebuffer::Framebuffer;

#[repr(C)]

pub struct BiosInfo {
    pub framebuffer: Framebuffer 
}

impl BiosInfo {
    pub fn new(framebuffer: Framebuffer) -> Self{
        BiosInfo {
            framebuffer
        }
    }
}