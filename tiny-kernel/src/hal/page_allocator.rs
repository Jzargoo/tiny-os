#[cfg(target_arch = "x86_64")]
use x86_64::{
    VirtAddr, structures::paging::{FrameAllocator, PageSize, PageTable}
};

pub const PAGE_SIZE: usize = 4096; // 4KB

pub struct BitmapAlloc{
    ptr_table: *mut PageTable
}

unsafe impl<T: PageSize> FrameAllocator<T> for BitmapAlloc {
    
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<T>> {
        todo!()
    }

}

#[cfg(target_arch = "x86_64")]
pub unsafe fn active_page_table_lvl4(physical_offset: VirtAddr) 
    -> &'static mut PageTable {

    use x86_64::registers::control::Cr3;

    let (active_ptr, _) = Cr3::read();

    let physical_address = active_ptr.start_address().as_u64();

    let virt = physical_offset + physical_address;

    let ptr_page_table_lvl4: *mut PageTable = virt.as_mut_ptr();

    return unsafe {
        &mut *ptr_page_table_lvl4   
    };

}