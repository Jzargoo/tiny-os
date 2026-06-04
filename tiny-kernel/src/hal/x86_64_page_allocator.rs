use x86_64::structures::paging::OffsetPageTable;

use x86_64::{
    VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PageTable}
};

const MAX_INDEX:u8 = 22; // MAX is 16GB 
const MIN_ORDER:u8 = 12; // MIN is 4KB or one page

struct BuddyNode{
    next: Option<*mut BuddyNode>
}

pub struct BuddyAlloc{
    ptr_table: OffsetPageTable<'static>,
    free_lists: [Option<*mut BuddyNode>; MAX_INDEX as usize],
    current_max_order: u8
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

#[allow(dead_code)]
impl BuddyAlloc {
    
    pub fn new(physical_offset: VirtAddr) -> Self{
        
        let pml4 = active_page_table_lvl4(physical_offset.clone());
        
        let of_pt = unsafe {
            OffsetPageTable::new( pml4 , physical_offset)
        };

        Self { 
            ptr_table: of_pt,
            free_lists: [Option::None as Option<*mut BuddyNode>; MAX_INDEX as usize],
            current_max_order: 0
         }
    }

    // Start_addr is the first byte of the region
    pub fn add_region(&mut self,start_addr:  *mut u8, mut size: usize) {
        
        let mut curr_addr = start_addr;

        while size != 0 {
            
            let order = calculate_order(size); // if it is 0 - min(4 page) 
            
            
            if order < 0 {
                panic!("an error in adding a region for BuddyAlloc. Region cutted on pages cannot be less that one page");
            } 
        

            let index = if order as usize >= MAX_INDEX as usize {
                MAX_INDEX as usize
            } else {
                order as usize
            };

            if self.current_max_order < index as u8 {
                self.current_max_order = index as u8;
            }

            unsafe {
                let new_node_ptr = curr_addr as *mut BuddyNode;

                let current_head = self.free_lists[index];
                
                new_node_ptr.write(
                    BuddyNode { next: current_head }
                );

                self.free_lists[index] = Some(new_node_ptr);
            
            };
            
            size -= 1 << (MIN_ORDER as usize + index);

            unsafe {
                curr_addr = curr_addr
                    .add(
                        1 <<  (MIN_ORDER as usize + index)
                    );
            };   

        }
    }


}

fn calculate_order(mut size: usize) -> isize{
        let mut x = 0;
        
        while  size  > 1 {
            x += 1;
            size /= 2;
        }

        x - MIN_ORDER as isize
}

unsafe impl<T: PageSize> FrameAllocator<T> for BuddyAlloc {
    
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        None
    }

}

impl <T: PageSize> FrameDeallocator<T> for BuddyAlloc {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<T>) {
        
    }
}