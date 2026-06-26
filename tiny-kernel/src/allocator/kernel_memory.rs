use core::ptr::null_mut;

const ALIGN_MASK: usize =15;

struct Slab {
    free_list_head: Option<*mut ListNode>,
    next: Option<*mut Slab>,    
}

struct ListNode {
    next: Option<*mut ListNode>,
}

#[repr(C, align(16))]
struct k_mem_cache_node {
    count: usize,
    first: *mut Slab,
}

#[repr(C, align(16))]
pub struct k_mem_cache {
    object_size: u16, // in bytes
    node: k_mem_cache_node,
}

impl k_mem_cache {
    pub const fn new(object_size: u16) -> Self{
        Self { 
            object_size,
            node: k_mem_cache_node{
                count: 0,
                first: null_mut()
            }
        }
    } 

    pub fn grow(&mut self, page_start_addr: *mut u8, page_size:usize) {
        
        let slab_ptr = page_start_addr as *mut Slab;

        let mut obj_curr_ptr = unsafe { (slab_ptr.add(1)) as *mut u8 };

        obj_curr_ptr = (((obj_curr_ptr as usize) + ALIGN_MASK) & !ALIGN_MASK) as *mut u8;

        let mut head: *mut ListNode = null_mut();
        let mut tail: *mut ListNode = null_mut();

        let page_end_addr = unsafe { page_start_addr.add(page_size) };

        while unsafe { obj_curr_ptr.add(self.object_size as usize) <= page_end_addr } {
            let curr_node_ptr = obj_curr_ptr as *mut ListNode;

            unsafe { (*curr_node_ptr).next = None };

            if head.is_null() {
                head = curr_node_ptr; 
            } else {
                unsafe { (*tail).next = Some(curr_node_ptr) };
            }

            tail = curr_node_ptr;

            obj_curr_ptr = unsafe { obj_curr_ptr.add(self.object_size as usize) } ;
        }

        unsafe {
            slab_ptr.write(
                Slab{
                    free_list_head: if head.is_null() { None } else { Some(head) },
                    next: None
                }
            );
        }

        if self.node.first.is_null() {
            self.node.first = slab_ptr;
            self.node.count = 1;
        } else {
            unsafe { (*slab_ptr).next = Some (self.node.first) } 
            self.node.first = slab_ptr;
            self.node.count += 1;
        }

    }
}

/*
#[repr(C, align(16))]
struct k_mem_cache_cpu {
    cache: *mut Slab,
    object_size: usize,
    freelist: *mut ListNode
}
*/

unsafe impl Send for k_mem_cache {}
unsafe impl Sync for k_mem_cache {}