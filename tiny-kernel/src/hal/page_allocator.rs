
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageSize{
    REGULAR,
    LARGE,
    HUGE,
}


#[derive(Debug, Clone, Copy)]
pub struct KernelMemRegion {
    pub start_paddr: u64,
    pub length_bytes: usize
}

#[derive(Debug)]
pub struct VirtPages{
    pub page_count: u8,
    pub page_size: PageSize,
    pub start_addr: u64
}

impl PageSize {
    pub fn bytes_from_page_size(page_size: PageSize) -> usize{
        match page_size {
            PageSize::REGULAR => crate::arch::pages::PAGE_SIZE_REGULAR,
            PageSize::LARGE => crate::arch::pages::PAGE_SIZE_LARGE,
            PageSize::HUGE => crate::arch::pages::PAGE_SIZE_HUGE,
        }
    }
}

impl VirtPages {
    pub fn new(page_count: u8, page_size: PageSize, start_addr: u64) -> Self{
        Self {
            page_count,
            page_size,
            start_addr
        }
    } 

    pub fn new_with_bytes(bytes: usize, page_size: PageSize, start_addr: u64) -> Self{
        // bytes per page
        let bpp = PageSize::bytes_from_page_size(page_size);

        Self {
            page_count: (bytes - 1 / bpp) as u8 + 1u8,
            page_size,
            start_addr 
        }
    }



    pub fn calc_bytes(&self) -> usize{
        PageSize::bytes_from_page_size(self.page_size) * self.page_count as usize 
    }
}


impl KernelMemRegion {
    pub const fn new(start_paddr: u64, length_bytes: usize) -> Self {
        Self { start_paddr, length_bytes }
    }
}

pub trait PageAllocator {
    fn allocate_pages(&mut self, count: u8, pg: PageSize) -> Option<VirtPages>;

    fn deallocate_pages(&mut self, pages: VirtPages);

    fn allocate_page(&mut self, pg: PageSize) -> Option<VirtPages> {
        self.allocate_pages(1, pg)
    }

    fn kernel_allocate_pages(&mut self, count: u8, pg: PageSize) -> Option<VirtPages>;
}