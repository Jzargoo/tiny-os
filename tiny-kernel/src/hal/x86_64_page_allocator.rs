use core::ptr::slice_from_raw_parts_mut;

use x86_64::PhysAddr;
use x86_64::structures::paging::{OffsetPageTable};

use x86_64::{
    VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PageTable}
};

use crate::hal::kernel_allocator::BumpAllocator;
use crate::hal::page_allocator::PageAllocator;
use crate::logger::LOGGER;

const MIN_ORDER:u8 = 12; // MIN is 4KB or one page
const MAX_ORDER:u8 = 20;


#[allow(dead_code)]
struct BuddyRoot {
    next: Option<*mut BuddyRoot>,
    start: usize,
    bitmap: *mut [bool],
    order: u8, 
}

#[allow(dead_code)]
pub struct BuddyAlloc{
    ptr_table: OffsetPageTable<'static>,
    buddy_root: Option<*mut BuddyRoot>,
    k_offset: u64
}


fn active_page_table_lvl4(physical_offset: VirtAddr) 
    -> &'static mut PageTable {

    use x86_64::registers::control::Cr3;

    let (active_ptr, _) = Cr3::read();

    let physical_address = active_ptr.start_address().as_u64();

    let virt = physical_offset + physical_address;

    let ptr_page_table_lvl4: *mut PageTable = virt.as_mut_ptr();

    return unsafe {
        &mut *ptr_page_table_lvl4   
    };
}

impl BuddyRoot {
    fn find_free_block(&mut self, generation: u8) -> Option<usize>{ 
        
        let bitmap =  unsafe {
            self.bitmap.as_mut().unwrap_or_else(
                || {
                    LOGGER.lock().write("The bitmap in the buddy root was not initialized");
                    panic!();
                }
            )
        };

        let idx = BuddyRoot::find_free_index_by_bitmap(generation, bitmap, 0)?;

        let block_size = 1 << (self.order - generation);

        let local_idx = idx - ((1 << generation) - 1);

        let offset = local_idx * block_size;

        Some(
            offset
        )
    }

    fn find_free_index_by_bitmap(target_generation: u8, bitmap: &mut[bool], sub_tree: usize) -> Option<usize> { 
        if sub_tree >= bitmap.len() as usize {
            return None;
        }

        let curr_gen = ((sub_tree + 1).ilog2()) as u8;

        if curr_gen == target_generation {
            if bitmap[sub_tree] {
                return None;
            } else {
                bitmap[sub_tree] = true;
                return Some(sub_tree);
            }
        } 
        
        let left_child_idx = (sub_tree * 2) + 1; 
        
        if let Some(idx) = BuddyRoot::find_free_index_by_bitmap(target_generation, bitmap, left_child_idx) {
            bitmap[sub_tree] = true;
            return Some(idx);
        }
        
        
        let right_child_idx = (sub_tree * 2) + 2;
        
        if let Some(idx) = BuddyRoot::find_free_index_by_bitmap(target_generation, bitmap, right_child_idx) {
            bitmap[sub_tree] = true;
            return Some(idx);
        }

        None
    }
}


#[allow(dead_code)]
impl BuddyAlloc {
    
    pub fn new(physical_offset: VirtAddr) -> Self{
        
        let pml4 = active_page_table_lvl4(physical_offset.clone());
        
        let of_pt = unsafe {
            OffsetPageTable::new( pml4 , physical_offset)
        };

        Self { 
            ptr_table: of_pt,
            buddy_root: None,
            k_offset: physical_offset.as_u64()
         }
    }

    pub fn add_region(&mut self,start_addr:  *mut u8, mut size: usize, kernel_alloc: &mut BumpAllocator) {
        
        let mut curr_addr = start_addr;

        while size != 0 {
            

            let mut order = size.ilog2() as u8;
            
            
            if order < MIN_ORDER {
                return;
            } else if order > MAX_ORDER{
                order = MAX_ORDER;
            }

            
            let root_ptr = kernel_alloc
                .k_alloc(
                    size_of::<BuddyRoot>(),
                    align_of::<BuddyRoot>(),
                )
                .expect("No memory") as *mut BuddyRoot;

            let leaves_count = 1 << (order - MIN_ORDER);

            let bitmap_len = (leaves_count * 2) - 1;

            let bitmap_ptr = kernel_alloc
                .k_alloc(
                    bitmap_len, 
                    align_of::<bool>()
                ).unwrap_or_else(
                    || {
                        LOGGER.lock().write("Out of kernel memory; however, there is an region");
                        panic!();
                    } 
                ) as *mut bool;


            
            unsafe {

                core::ptr::write_bytes(bitmap_ptr, 0u8, bitmap_len);

                let bitmap: *mut [bool] = slice_from_raw_parts_mut(bitmap_ptr, bitmap_len);

                root_ptr.write(
                    BuddyRoot { 
                        next: self.buddy_root,
                        start: curr_addr as usize,
                        bitmap: bitmap,
                        order
                    }
                );
            }

            self.buddy_root = Some(root_ptr);

            
            let block_size = 1usize << (order as usize);

            curr_addr = unsafe { curr_addr.add(block_size) };
                
            size -= block_size;
        }
    }

    fn allocate_bytes(&mut self, frame_size_bytes: usize) 
        -> Option<super::page_allocator::PhysFrame> {

        let order = calculate_order_ceil(frame_size_bytes as usize);

        if order < MIN_ORDER{
            LOGGER.lock().write("requsted less than one page");
            return None;
        } else if order > MAX_ORDER {
            LOGGER.lock().write("requsted more than root can contain");
            return None;
        }

           
        let mut current = self.buddy_root;

        while let Some(node_ptr) = current {
            unsafe {

                let root= node_ptr.as_mut().unwrap_or_else(
                    || {
                        LOGGER.lock().write("Error: Invalid buddy root pointer");
                        panic!();
                    } 
                );
                
                if order <= root.order {
 
                    if let Some(block) = root.find_free_block(root.order - order) {
 
                        let v_start = (root.start + block) as u64;
                        
                        let start = v_start - self.k_offset;

                        let frame = super::page_allocator::PhysFrame {
                            start_address: start as usize,
                            length_bytes: frame_size_bytes as usize
                        };

                        return Some(frame);
                    }
                }

                current = root.next
            }
        }
        None // cannot find any roots or space there
    }

}


fn calculate_order_ceil(size: usize) -> u8{
    if size <= 1 {
        0
    } else {
        ((size - 1).ilog2() + 1) as u8
    }
}


unsafe impl<T: PageSize> FrameAllocator<T> for BuddyAlloc {
    
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        
        self.allocate_bytes(T::SIZE as usize).map(|frame| {
            x86_64::structures::paging::PhysFrame::containing_address(
                PhysAddr::new(frame.start_address as u64)
            )
        })
    }
}

impl <T: PageSize> FrameDeallocator<T> for BuddyAlloc {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<T>) {
        
    }
}

impl PageAllocator for BuddyAlloc {
    

    fn allocate_pages(&mut self, count: usize, pg: super::page_allocator::PageSize) 
        -> Option<super::page_allocator::PhysFrame> {

        self.allocate_bytes(count * pg.size())
    }

    fn deallocate_frame(&mut self, frame: crate::hal::page_allocator::PhysFrame) {
        
    }
}