use core::alloc::{GlobalAlloc, Layout};

mod kernel_memory;

#[repr(C, align(16))]
pub struct SlubAllocator {
    // caches: [kernel_memory::k_mem_cache; 10], // 8 16 32 64 128 256 512 1024 2048 4096
}

unsafe impl GlobalAlloc for SlubAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
