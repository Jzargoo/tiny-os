use crate::{acpi::xsdt::Xsdt, hal::{framebuffer::Framebuffer, kernel_allocator::BumpAllocator, page_allocator::PageAllocator}};

#[repr(C)]
pub struct BiosInfo<'a> {
    pub framebuffer: Framebuffer,
    pub page_allocator: &'a mut dyn PageAllocator,
    pub kernel_alloc: BumpAllocator,
    pub xsdt: Xsdt
}

impl <'a> BiosInfo<'a> {
    pub fn new(framebuffer: Framebuffer, bump_alloc: BumpAllocator, page_alloc: &'a mut dyn PageAllocator, xsdt: Xsdt) -> Self{
        BiosInfo {
            framebuffer,
            kernel_alloc: bump_alloc,
            page_allocator: page_alloc,
            xsdt
        }
    }
}