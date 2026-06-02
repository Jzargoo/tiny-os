use crate::hal::framebuffer::Framebuffer;

#[repr(C)]

pub struct BiosInfo {
    pub framebuffer: Framebuffer,
    // pub page_allocator: PageAllocator,
    pub phys_memory_offset: u64
}

impl BiosInfo {
    pub fn new(framebuffer: Framebuffer, phys_addr: u64) -> Self{
        BiosInfo {
            framebuffer,
            phys_memory_offset: phys_addr
        }
    }
}