use core::ptr::null_mut;

use crate::hal::page_allocator::{PageAllocator, PageSize::REGULAR};

const ALIGN_MASK: usize =15;

pub struct Slab {
    pub free_list_head: Option<*mut ListNode>,
    pub  next: Option<*mut Slab>,    
}

pub struct ListNode {
    pub next: Option<*mut ListNode>,
}

#[repr(C, align(16))]
pub struct k_mem_cache_node {
    count: usize,
    pub active: *mut Slab,
    pub partial_first: *mut Slab
}

#[repr(C, align(16))]
pub struct k_mem_cache {
    pub object_size: u16, // in bytes
    pub node: k_mem_cache_node,
}

impl k_mem_cache {
    pub const fn new(object_size: u16) -> Self{
        Self { 
            object_size,
            node: k_mem_cache_node{
                count: 0,
                partial_first: null_mut(),
                active: null_mut()
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

        if self.node.active.is_null() {
            self.node.active = slab_ptr;
            self.node.count = 0; // occured only when partial first is null so it is empty
        } else {
            unsafe { (*self.node.active).next = Some(self.node.partial_first) };
            
            self.node.partial_first = self.node.active;

            self.node.active = slab_ptr;
            
            self.node.count += 1;
        }

    }

    pub fn change_active(&mut self, alloc: *mut dyn PageAllocator) {
        if self.node.partial_first.is_null() {
            let pages = unsafe { (*alloc).kernel_allocate_page(REGULAR)};

            if let Some(page) = pages {
                self.grow(page.start_addr as *mut u8, page.calc_bytes());
            } else {
                return; // we can do nothing 
            }

            self.node.partial_first = null_mut()
        } else {
            self.node.active = self.node.partial_first;
            self.node.partial_first = unsafe{  
                let next = (*self.node.active).next;

                (*self.node.active).next = None;

                match next {
                    None => null_mut(),
                    Some(x) => x
                }
            };
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