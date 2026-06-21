use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB}};

use crate::hal::{buddy_mem_manager::BuddyManager, page_allocator::{PageAllocator, PageSize, VirtPages}};

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
    fn update_page_table(&mut self, pstart_addr: u64, vstart_addr: u64, page_size: PageSize, count: u8){


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
    unsafe fn deallocate_frame(&mut self, _frame: x86_64::structures::paging::PhysFrame<T>) {
        
    }
}

impl <T: x86_64::structures::paging::PageSize> FrameDeallocator<T> for &mut BuddyManager {
    unsafe fn deallocate_frame(&mut self, _frame: x86_64::structures::paging::PhysFrame<T>) {
        
    }
}


impl PageAllocator for PageAllocationMapper {

    fn allocate_pages(&mut self, count: u8, pg: PageSize) 
        -> Option<VirtPages> {

        if let Some(frame) = 
            self.buddy_manager.allocate_bytes(count as usize * PageSize::bytes_from_page_size(pg)
        ) {
            self.update_page_table(
                frame.start_paddr - self.k_offset,
                frame.start_paddr,
                pg, count
            );
            
            Some(
                VirtPages{
                    start_addr: frame.start_paddr - self.k_offset,
                    page_count: count,
                    page_size: pg
                }
            )
        } else {
            None
        }
    }

    fn deallocate_pages(&mut self, frame: VirtPages) {
        todo!()        
    }
    
    fn kernel_allocate_pages(&mut self, count: u8, pg: PageSize) -> Option<VirtPages> {
        
        if let Some(frame) = 
            self.buddy_manager.allocate_bytes(count as usize * PageSize::bytes_from_page_size(pg)
        ) {
            // A simple but important difference between invokes
            self.update_page_table(
                frame.start_paddr,
                frame.start_paddr,
                pg, count
            );
            
            Some(
                VirtPages{
                    start_addr: frame.start_paddr - self.k_offset,
                    page_count: count,
                    page_size: pg
                }
            )
        } else {
            None
        }

    }
    
    fn kernel_deallocate_pages(&mut self, frame: VirtPages) {
        todo!()
    }
}
