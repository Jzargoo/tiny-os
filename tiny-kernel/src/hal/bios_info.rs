use crate::hal::{framebuffer::Framebuffer, kernel_allocator::BumpAllocator};

#[repr(C)]

pub struct BiosInfo {
    pub framebuffer: Framebuffer,
    // pub page_allocator: PageAllocator,
    pub kernel_alloc: BumpAllocator,
    pub phys_memory_offset: u64
}

impl BiosInfo {
    pub fn new(framebuffer: Framebuffer, phys_addr: u64, bumpAlloc: BumpAllocator) -> Self{
        BiosInfo {
            framebuffer,
            phys_memory_offset: phys_addr,
            kernel_alloc: bumpAlloc
        }
    }
}