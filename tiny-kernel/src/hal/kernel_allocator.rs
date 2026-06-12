pub struct BumpAllocator {
    start: usize,
    end: usize,
    current: usize,
}

impl BumpAllocator {
    pub fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            end: start + size,
            current: start,
        }
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let mut addr = (self.current + align - 1) & !(align - 1);

        if addr + size > self.end {
            return None;
        }

        self.current = addr + size;

        Some(addr as *mut u8)
    }

}