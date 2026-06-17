
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageSize{
    REGULAR,
    LARGE,
    HUGE,
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