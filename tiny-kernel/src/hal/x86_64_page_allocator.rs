use x86_64::structures::paging::OffsetPageTable;

use x86_64::{
    VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PageTable}
};

const MIN_ORDER:u8 = 12; // MIN is 4KB or one page

#[allow(dead_code)]
struct BuddyNode{
    left: Option<*mut BuddyNode>, 
    right: Option<*mut BuddyNode>
}

#[allow(dead_code)]
struct BuddyRoot {
    next: Option<*mut BuddyRoot>,
    tree: BuddyNode,
    rel_order: u8, // order of the size relative to min (4 kb)
}

#[allow(dead_code)]
pub struct BuddyAlloc{
    ptr_table: OffsetPageTable<'static>,
    buddy_root: Option<*mut BuddyRoot>
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
            buddy_root: None
            // bitmap: core::array::from_fn(|_| None)
         }
    }

    pub fn add_region(&mut self,start_addr:  *mut u8, mut size: usize) {
        
        let mut curr_addr = start_addr;

        while size != 0 {
            
            let order = calculate_order(size); // if it is 0 - min(4 page) 
            
            
            if order < 0 {
                panic!("an error in adding a region for BuddyAlloc. Region cutted on pages cannot be less that one page");
            } 

            
            let bn = BuddyNode{
                left: None,
                right: None
            };
            
            unsafe {
                let curr_addr =  curr_addr as *mut BuddyRoot;
                
                curr_addr.write(
                    BuddyRoot { 
                        next: self.buddy_root, 
                        tree: bn, 
                        rel_order: order as u8 
                    }
                );
            }

            self.buddy_root = Some(
                    curr_addr as *mut BuddyRoot
            );

            let block_size = 1usize << ((order as usize) + (MIN_ORDER as usize));

            unsafe {
                curr_addr = curr_addr.add(block_size);
            }

            size -= block_size;

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