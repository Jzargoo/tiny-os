use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size1GiB, Size2MiB, Size4KiB, mapper::MapperFlush}};

use crate::{hal::{buddy_mem_manager::BuddyManager, page_allocator::{PageAllocator, PageSize, VirtPages}}, println};

#[derive(Debug)]
pub struct PageAllocationMapper {
    buddy_manager: BuddyManager,
    ptr_table: OffsetPageTable<'static>,
    k_offset: u64 
}



impl PageAllocationMapper {
    pub fn new(physical_offset: u64, buddy: BuddyManager) -> Self {
        let pml4 = active_page_table_lvl4(physical_offset.clone());
        
        let of_pt = unsafe {
            OffsetPageTable::new( pml4 , VirtAddr::new(physical_offset))
        };

        Self {
            buddy_manager: buddy,
            ptr_table: of_pt,
            k_offset: physical_offset
        }        
    }
}

impl PageAllocationMapper {
    fn  remove_from_page_table(&mut self, pages: VirtPages) {

        let free = |mapper: &mut OffsetPageTable| match pages.page_size {
            PageSize::REGULAR => free_pages_of_type::<Size4KiB, _>(mapper, pages),
            PageSize::LARGE   => free_pages_of_type::<Size2MiB, _>(mapper, pages),
            PageSize::HUGE    => free_pages_of_type::<Size1GiB, _>(mapper, pages),
        };

        free(&mut self.ptr_table);
    }
}

fn free_pages_of_type<S,T>(mapper:&mut T, pages: VirtPages) 
    where
        S: x86_64::structures::paging::page::PageSize,
        T: Mapper<S>, {

    for i in 0..pages.page_count as u64 {
            
        let offset = S::SIZE * i;
            
        let vaddr = VirtAddr::new(pages.start_addr + offset);
        
        if let Ok(page) = Page::<S>::from_start_address(vaddr) {
            if let Ok((_frame, flush)) = mapper.unmap(page) {
                flush.flush();
            }
        }
    }
}

impl PageAllocationMapper {
    fn add_into_page_table(&mut self, pstart_addr: u64, vstart_addr: u64, page_size: PageSize, count: u8){


        let vaddr = VirtAddr::new(vstart_addr);

        let paddr = PhysAddr::new(pstart_addr);

        for i in 0..count {
            let offset = PageSize::bytes_from_page_size(page_size) as u64 * i as u64;

            self.map_regular_page(vaddr + offset, paddr + offset);
        }

    }

    fn map_regular_page(&mut self, vsa: VirtAddr, psa: PhysAddr) {

        let PageAllocationMapper {
            ref mut ptr_table,
            ref mut buddy_manager,
            ..
        } = *self;
        
        let page = Page::<Size4KiB>::containing_address(vsa);

        let frame = 
            x86_64::structures::paging::PhysFrame::<Size4KiB>::containing_address(psa);

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        let map_to_result = unsafe {
            ptr_table.map_to(page, frame, flags, buddy_manager)   
        };

        match map_to_result {
            Ok(tlb_flush) => tlb_flush.flush(),
            Err(err) => panic!("Error occured while mapping was made! {:?}", err)
        }
    }
}


fn active_page_table_lvl4(physical_offset: u64) 
    -> &'static mut PageTable {

    use x86_64::registers::control::Cr3;
    
    let virt_offset = VirtAddr::new(physical_offset);

    let (active_ptr, _) = Cr3::read();

    let physical_address = active_ptr.start_address().as_u64();

    let virt = virt_offset + physical_address;

    let ptr_page_table_lvl4: *mut PageTable = virt.as_mut_ptr();

    return unsafe {
        &mut *ptr_page_table_lvl4   
    };
}

unsafe impl<T: x86_64::structures::paging::PageSize> FrameAllocator<T> for BuddyManager {
    
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        
        self.allocate_bytes(T::SIZE as usize).map(|frame| {
            x86_64::structures::paging::PhysFrame::containing_address(
                PhysAddr::new(frame.start_paddr as u64)
            )
        })
    }
}

unsafe impl<T: x86_64::structures::paging::PageSize> FrameAllocator<T> for &mut BuddyManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        self.allocate_bytes(T::SIZE as usize).map(|frame| {
            x86_64::structures::paging::PhysFrame::containing_address(
                PhysAddr::new(frame.start_paddr as u64)
            )
        })
    }
}

impl <T: x86_64::structures::paging::PageSize> FrameDeallocator<T> for BuddyManager {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<T>) {
        let _ = self.deallocate_bytes(frame.start_address().as_u64(), frame.size() as usize);
    }
}

impl <T: x86_64::structures::paging::PageSize> FrameDeallocator<T> for &mut BuddyManager {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<T>) {
        let _ = self.deallocate_bytes(frame.start_address().as_u64(), frame.size() as usize);
    }
}


impl PageAllocator for PageAllocationMapper {

    fn allocate_pages(&mut self, count: u8, pg: PageSize) 
        -> Option<VirtPages> {

        if let Some(frame) = 
            self.buddy_manager.allocate_bytes(count as usize * PageSize::bytes_from_page_size(pg)
        ) {
            self.add_into_page_table(
                frame.start_paddr,
                frame.start_paddr,
                pg, count
            );
            
            Some(
                VirtPages{
                    start_addr: frame.start_paddr,
                    page_count: count,
                    page_size: pg
                }
            )
        } else {
            None
        }
    }

    fn deallocate_pages(&mut self, frame: VirtPages) {
        let bytes = frame.page_count as usize * PageSize::bytes_from_page_size(frame.page_size);  
        
        if let Ok(_) = self.buddy_manager.deallocate_bytes(frame.start_addr, bytes) {
            self.remove_from_page_table(frame);
            #[cfg(debug_assertions)]
            println!("Successfully removed pages in mapper and bytes in buddy system!")
        } else {
            panic!("Returned an error in deallocating page table. CANNOT catch errors")
        }

    }

    fn kernel_deallocate_pages(&mut self, frame: VirtPages) {
        let bytes = frame.page_count as usize * PageSize::bytes_from_page_size(frame.page_size);  
        
        if let Ok(_) = self.buddy_manager.deallocate_bytes(frame.start_addr, bytes) {
            #[cfg(debug_assertions)]
            println!("Successfully removed pages in mapper and bytes in buddy system!")
        } else {
            panic!("Returned an error in deallocating page table. CANNOT catch errors")
        }

    }
    
    fn kernel_allocate_pages(&mut self, count: u8, pg: PageSize) -> Option<VirtPages> {
        
        if let Some(frame) = 
            self.buddy_manager.allocate_bytes(count as usize * PageSize::bytes_from_page_size(pg)
        ) {
            
            // There is not sense in page allocation for page table in kernel because it has been already allocated 

            Some(
                VirtPages{
                    start_addr: frame.start_paddr + self.k_offset,
                    page_count: count,
                    page_size: pg
                }
            )

        } else {
            None
        }

    }
}