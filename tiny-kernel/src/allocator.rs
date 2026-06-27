use core::{alloc::{GlobalAlloc, Layout}, ptr::null_mut, sync::atomic::{AtomicU8, Ordering::Relaxed}};

use spin::Mutex;

use crate::{allocator::kernel_memory::k_mem_cache, hal::page_allocator::PageAllocator};

mod kernel_memory;

#[repr(C, align(16))]
pub struct SlubAllocator {
    caches: [Mutex<k_mem_cache>; 9], // 8 16 32 64 128 256 512 1024 2048
    page_alloc: 
        Mutex<
            Option<*mut dyn PageAllocator>
        >,
    incr_factor: AtomicU8,
    start_obj_size: AtomicU8
}

unsafe impl GlobalAlloc for SlubAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let level_cache = match self.evaluate_level_cache(layout.size())  {
            Some(x) => x,
            None => return null_mut()
        };

        let mut cache = self.caches[level_cache].lock();
        
        let page_alloc_ptr = self.page_alloc.lock().expect("expected initiated page allocator");

        if cache.node.first.is_null() {
            

            let new_page = unsafe { 
                (*page_alloc_ptr).kernel_allocate_page(crate::hal::page_allocator::PageSize::REGULAR) 
            };

            match new_page {
                None => return null_mut(),
                Some(vp) => cache.grow(vp.start_addr as *mut u8, vp.calc_bytes()),
            }    
        }

        let mut allocated_ptr: *mut u8 = null_mut();

        let mut slab_ptr = cache.node.first;

        while !slab_ptr.is_null() {
            unsafe {
                if let Some(node_ptr) = (*slab_ptr).free_list_head {
                    (*slab_ptr).free_list_head = (*node_ptr).next;
                    allocated_ptr = node_ptr as *mut u8;
                    break;
                } else {
                    slab_ptr = match (*slab_ptr).next {
                        Some(next_ptr) => next_ptr,
                        None => null_mut(),
                    };
                }
            }
        }

        if allocated_ptr.is_null() {
            let new_page = unsafe { 
                (*page_alloc_ptr).kernel_allocate_page(crate::hal::page_allocator::PageSize::REGULAR) 
            };

            match new_page {
                None => return null_mut(), // Совсем кончилась память в системе
                Some(vp) => {
                    cache.grow(vp.start_addr as *mut u8, vp.calc_bytes());
            
                    let fresh_slab_ptr = cache.node.first;
                    
                    unsafe {
                        if let Some(node_ptr) = (*fresh_slab_ptr).free_list_head {
                                (*fresh_slab_ptr).free_list_head = (*node_ptr).next;
                                allocated_ptr = node_ptr as *mut u8;
                        }
                    }
                }
            }
        }

        allocated_ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!("Deallocation was not implemented!")
    }
}

fn round_power_two(size: usize) -> usize{
    
    if size <= 1 {
        return 1;
    }

    let power = (size - 1).ilog2() + 1;

    1 << power
}

impl SlubAllocator { 

    fn evaluate_level_cache(&self,size: usize) -> Option<usize> {

        let obj_size = self.start_obj_size.load(Relaxed) as usize;
        
        let factor = self.incr_factor.load(Relaxed) as usize;
        
        if size > obj_size * factor.pow(self.caches.len() as u32 - 1u32) {
            return None;
        } else if size <= obj_size {
            return Some(0);
        }

        let rounded = round_power_two(size);

        let bit_difference = rounded.trailing_zeros() as usize - obj_size.trailing_zeros() as usize;

        let factor_step = factor.trailing_zeros() as usize;

        if factor_step == 0 {
            return None;
        }

        let index = bit_difference / factor_step;

        if index >= self.caches.len() {
            return None;
        } else {
            return Some(index);
        }

    }

    pub const fn default() -> Self{
        
        let caches = [
            Mutex::<k_mem_cache>::new(k_mem_cache::new(8)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(16)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(32)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(64)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(128)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(256)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(512)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(1024)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(2048))
        ];

        Self {
            caches,
            page_alloc: Mutex::new(None),
            incr_factor: AtomicU8::new(2),
            start_obj_size: AtomicU8::new(8)
        }

    }

    pub fn set_page_allocator(&self, alloc: *mut dyn PageAllocator){
        let mut alloc_guard = self.page_alloc.lock();

        (*alloc_guard) = Some(alloc)
    }
}

unsafe impl Send for SlubAllocator {}
unsafe impl Sync for SlubAllocator {}