
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageSize{
    REGULAR(usize),
    LARGE(usize),
    HUGE(usize),
}

impl PageSize {
    pub fn size(&self) -> usize {
        match self {
            PageSize::REGULAR(size) => *size,
            PageSize::LARGE(size) => *size,
            PageSize::HUGE(size) => *size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysFrame{
    pub start_address: usize,
    pub length_bytes: usize
}

pub trait PageAllocator {
     fn allocate_pages(&mut self, count: usize, pg: PageSize) -> Option<PhysFrame>;
     fn deallocate_frame(&mut self, frame: PhysFrame);
}