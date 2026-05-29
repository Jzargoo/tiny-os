pub const PAGE_SIZE: usize = 4096; // 4KB

#[repr(C)]
pub struct Page {
    page: *mut u8,
    next: Option<*mut Page>
}

pub struct Entry{
    pub base: usize,
    pub length: usize,
}

pub struct PageAllocator {
    page_start: Page,
}

impl PageAllocator {
    pub fn new(entries: &'static [Entry]) -> Self {
               
        
    }
}