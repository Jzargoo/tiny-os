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


    // The `grow` method is called in exactly two scenarios:

    // 1. Initial boot: `self.node.active` is completely null. We simply assign the new 
    //    slab to `active`. No pointers are dereferenced, preventing an early kernel page fault.

    // 2. Slow Path / Out of Memory in Active: The current active slab is 100% full. Instead of 
    //    wasting CPU cycles linking this fully saturated slab to the `partial` list, we deliberately 
    //    "abandon" it by overwriting `self.node.active` with the new slab. 

    // Even though the cache manager temporarily "forgets" this full slab, its page boundaries 
    // remain intact. The moment `dealloc` is called on any object residing inside this drifting slab, 
    // the kernel calculates its metadata pointer via alignment. 
    // Once `dealloc` frees the first object, that slab will automatically resurrect itself and link 
    // back into the `partial_first` chain. Moreover, we only forget about active so we everytime
    // have next = None. IT IS IMPORTANT otherwise we could potentially save a slab with a next that pointee on incorrect data.


    /*
    WHAT SHOULD WE TRACK:
    
            if !cache.node.partial_first.is_null() {
                unsafe { (*slab_ptr).next = Some(cache.node.partial_first) }
            } 
            
            we did not update next (in the freed slub) if partial list is empty, so if we forget a slub with next -> some slub.
            This situation can lead to page fault
            
    */  

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
                /*
                    * ARCHITECTURAL FIX: Preventing "Ghost" Slab Proliferation during OOM
                    *
                    * SCENARIO THAT BROKE THE OLD MODEL:
                    * 1. We have exactly one active slab, and it becomes 100% full.
                    * 2. An `alloc` occurs -> `grow` fails due to physical Out-of-Memory (OOM).
                    * 3. In the old version, we kept this full slab as `active`.
                    * 4.  `dealloc` occurs on a different drifting slab, making it the new head of `partial_first`.
                    * 5. A subsequent `dealloc` occurs on this same active page -> it frees 1 slot and 
                    * immediately pushes ITSELF into the `partial_first` list as well.
                    * 6. Now the same physical slab exists SIMULTANEOUSLY as `active` and `partial_first`.
                    * 7. When allocations resume, the overlap causes the cache state to desynchronize, eventually 
                    * forcing `change_active` to load a completely FULL slab back into `active`; however we have memory from step 4, resulting in an
                    * erroneous `null_mut()` (OOM) return even though other partial slabs have free space.
                    *
                    * THE FIX:
                    * If `grow` fails to allocate a regular page, we forcibly set `self.node.active = null_mut()`.
                    * This separates the lifecycle completely. If a `dealloc` happens later, the slab is pushed
                    * *only* to `partial_first`, completely avoiding any cross-contamination or "ghost" duplicates.
                    * Thus, `active` being NULL is a perfectly valid state during runtime OOM conditions.    
                */
                self.node.active = null_mut() 
            }
            
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