use crate::hal::framebuffer::Framebuffer;

#[repr(C)]

pub struct BiosInfo {
    pub framebuffer: Framebuffer,
    // pub page_allocator: PageAllocator,
}

impl BiosInfo {
    pub fn new(framebuffer: Framebuffer) -> Self{
        BiosInfo {
            framebuffer
        }
    }
}