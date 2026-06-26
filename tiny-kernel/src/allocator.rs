use core::alloc::{GlobalAlloc, Layout};

use spin::Mutex;

use crate::{allocator::kernel_memory::k_mem_cache};

mod kernel_memory;

#[repr(C, align(16))]
pub struct SlubAllocator {
    caches: [Mutex<k_mem_cache>; 10], // 8 16 32 64 128 256 512 1024 2048 4096
}

unsafe impl GlobalAlloc for SlubAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}

impl SlubAllocator{ 
    pub const fn new() -> Self{
        
        let caches = [
            Mutex::<k_mem_cache>::new(k_mem_cache::new(8)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(16)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(32)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(64)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(128)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(256)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(512)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(1024)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(2048)),
            Mutex::<k_mem_cache>::new(k_mem_cache::new(4096)),
        ];

        Self {
            caches
        }

    }

    pub fn add_page(&mut self, page_start: u64, page_size: usize, level_cache:u8) {
        if level_cache as usize >= self.caches.len() {
            return;
        } 

        let mut cache = self.caches[level_cache  as usize].lock();

        cache.grow(page_start as *mut u8, page_size);
    }
}