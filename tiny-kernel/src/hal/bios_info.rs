use crate::{acpi::{table_registry::TableRegistry}, hal::{addresses::PhysicalAddress, framebuffer::Framebuffer, kernel_allocator::BumpAllocator, page_allocator::PageAllocator}};

#[repr(C)]
pub struct BiosInfo<'a, P:PhysicalAddress> {
    pub framebuffer: Framebuffer,
    pub page_allocator: &'a mut dyn PageAllocator,
    pub kernel_alloc: BumpAllocator,
    pub acpi_tables: TableRegistry<P>
}

impl <'a, P: PhysicalAddress> BiosInfo<'a, P> {
    pub fn new(

        framebuffer: Framebuffer,
        
        bump_alloc: BumpAllocator,
        
        page_alloc: &'a mut dyn PageAllocator, 
        
        acpi_tables: TableRegistry<P>

    ) -> Self{

        BiosInfo {
            framebuffer,
            kernel_alloc: bump_alloc,
            page_allocator: page_alloc,
            acpi_tables
        }
    }
}