use limine::memmap::{Entry, MEMMAP_USABLE};
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB}};

use crate::{arch::x86_64::boot::limine::limine_requests::{HHDM, MEMMAP}, hal::{KERNEL_HEAP_SIZE, buddy_mem_manager::BuddyManager, kernel_allocator::BumpAllocator}, println};

pub fn memmap_init(alloc: &mut BuddyManager, offset: u64) -> Option<BumpAllocator> {
    
    if let Some(memmap) = MEMMAP.response() {
        
        println!("Initializing memory map entry!");

        
        let entries = memmap.entries();
        
        let mut kernel_alloc = init_kernel_alloc(entries, offset);
        
        for entry in entries {
            if let Some(k_alloc) = kernel_alloc.as_mut() && entry.type_ == MEMMAP_USABLE {

                let mut len = entry.length as usize;
                let mut base = entry.base;

                if k_alloc.start == base as usize {
                    base += KERNEL_HEAP_SIZE as u64;
                    len -= KERNEL_HEAP_SIZE
                }

                if len == 0 {
                    continue;
                }


                #[cfg(debug_assertions)]
                println!("Memmap entry has base {} and length {}", base, len);

                alloc.add_region(
                    base as *mut u8, 
                    len,
                    k_alloc
                );
            }
        }

        kernel_alloc
    
    } else {
        println!("No memory map available!");
        None
    }
}

fn init_kernel_alloc(entries: &[&Entry], offset: u64) -> Option<BumpAllocator> {
    for entry in entries {
        if entry.type_ == MEMMAP_USABLE && entry.length as usize >= KERNEL_HEAP_SIZE {
            return Some(BumpAllocator::new(entry.base as usize + offset as usize, KERNEL_HEAP_SIZE));
        }
    }
    None
}


pub fn hhdm_init() -> Option<u64>{
    if let Some(resp) = HHDM.response() {
        Some(
            resp.offset
        )
    } else {
        None
    }
}

pub fn pci_mechanism_init<M,A>(
    mapper: &mut M,
    alloc: &mut A,
    bus: usize,
    hhdm: u64,
    phys_base: u64,
) where M: Mapper<Size4KiB>, A: FrameAllocator<Size4KiB> {
    
    let size = bus as u64 * 1024 * 1024; 

    mmio_init::<M,A>(
        mapper, alloc, size, hhdm, phys_base,
        PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::NO_CACHE | PageTableFlags::NO_EXECUTE
    );
}

pub fn lapic_mmio_init<M,A>(
    mapper: &mut M,
    alloc: &mut A,
    hhdm: u64,
    phys_base: u64,
) where M: Mapper<Size4KiB>, A: FrameAllocator<Size4KiB> {
    
    mmio_init(
        mapper, 
        alloc, 
        4 * 1024, 
        hhdm, 
        phys_base,
        PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::NO_CACHE | PageTableFlags::NO_EXECUTE
    );
}


pub fn mmio_init<M,A>(
    mapper: &mut M,
    alloc: &mut A,
    size: u64,
    hhdm: u64,
    phys_base: u64,
    flags: PageTableFlags
) where M: Mapper<Size4KiB>, A: FrameAllocator<Size4KiB> {

    let phys_addr = PhysAddr::new(phys_base);
    let virt_addr = VirtAddr::new(hhdm + phys_base);

    let start_page: Page<Size4KiB> = Page::containing_address(virt_addr);
    let end_page: Page<Size4KiB> = Page::containing_address(virt_addr + size);

    for page in Page::range_inclusive(start_page, end_page) {
        let frame = PhysFrame::containing_address(
            phys_addr + (page.start_address() - virt_addr)
        );
        
        if let Ok(_existing) = mapper.translate_page(page) {
            
            unsafe {
            
                if let Ok(flush) = mapper.update_flags(page, flags) {
                    flush.flush();
                }
            
            }

        } else {
        
            unsafe {
            
                mapper
                    .map_to(page, frame, flags, alloc)
                    .expect("Failed to map ECAM page")
                    .flush(); 
            
            }

        }

    }

}