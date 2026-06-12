use x86_64::structures::paging::OffsetPageTable;

use x86_64::{
    VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PageTable}
};

use crate::logger::LOGGER;

const MIN_ORDER:u8 = 12; // MIN is 4KB or one page
const MAX_ORDER:u8 = 20;


#[allow(dead_code)]
struct BuddyRoot {
    next: Option<*mut BuddyRoot>,
    start: usize,
    order: u8, 
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
         }
    }

    pub fn add_region(&mut self,start_addr:  *mut u8, mut size: usize) {
        
        let mut curr_addr = start_addr;

        while size != 0 {
            
            let mut order = calculate_order(size);
            
            
            if order < MIN_ORDER {

                let block_size = 1usize << (order as usize);

                curr_addr = unsafe { curr_addr.add(block_size) };
                
                size -= block_size;

                continue;
            
            } else if order > MAX_ORDER{
                order = MAX_ORDER;
            }
            
            let bn = BuddyNode{
                left: None,
                right: None,
                state: State::FREE
            };

            unsafe {
                

                let root_ptr = curr_addr as *mut BuddyRoot;
                
                root_ptr.write(
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

            let block_size = 1usize << (order as usize);

            unsafe {
                curr_addr = curr_addr.add(block_size);
            }

            size -= block_size;

        }
    }

}

fn split(buddy_node: &mut BuddyNode, order: u8) { // order is the order of the buddy node

    if order < MIN_ORDER {
        LOGGER.lock().write("[ERROR] Was asked less than one page( the minimal size of the block )");
        panic!()
    }

    buddy_node.state = State::SPLITTED;
    let ptr = buddy_node as *mut BuddyNode;
    buddy_node.left = Some (
        ptr
    );

    buddy_node.right = Some (
        unsafe { ptr.byte_add(1 << (order - 1)) }
    );
} 

fn find_free_index(root: BuddyRoot, order: u8) -> Option<*mut u8> {
    
    let root_order = root.rel_order;
    
    let index = root_order - order;

    let bitmap = unsafe { root.bitmap.as_ref().expect("The bitmap array does not exist") };

    let start_index = (1 << index) - 1;
    let end_index = (1 << (index + 1)) - 1;

    for idx in start_index..end_index {
        if bitmap[idx] {
            let mut node = root
            let mut k = idx;
            while k != 0 {

                k /= 2;
            }
        } 
   }


    None
}

fn calculate_order(mut size: usize) -> u8{
        let mut x = 0;
        
        while  size  > 1 {
            x += 1;
            size /= 2;
        }

        x
}


unsafe impl<T: PageSize> FrameAllocator<T> for BuddyAlloc {
    
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        
        let frame_size_bytes = T::SIZE;

        let order = calculate_order(frame_size_bytes as usize);

        if let Some (root) = self.buddy_root{ 
            let mut current = self.buddy_root;

            while let Some(mut node_ptr) = current {
                unsafe {

                    let node = node_ptr.as_ref().expect("Expected a correct reference");

                    if node.rel_order >= order {    
                        // return Some(split()); 
                    }

                    current = node.next;
                }
            }

            None // cannot find 
        } else {
            None // Region is not contained in alloc
        }
    }

}

impl <T: PageSize> FrameDeallocator<T> for BuddyAlloc {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<T>) {
        
    }
}