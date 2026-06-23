use core::fmt::Write;

use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, memmap::{Entry, MEMMAP_USABLE}, request::{
        EntryPointRequest, FramebufferRequest, HhdmRequest, MemmapRequest, StackSizeRequest
    }
};

use crate::{
    arch::x86_64::page_allocator::PageAllocationMapper, hal::{    
        KERNEL_HEAP_SIZE, bios_info::BiosInfo, buddy_mem_manager::BuddyManager, framebuffer::Framebuffer, kernel_allocator::BumpAllocator
    }, kernel_main, logger::LOGGER, println};


    
#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();



#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section=".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();



#[used]
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(1024 * 1024 * 16);



#[unsafe(link_section = ".requests")]
pub static HHDM: HhdmRequest = HhdmRequest::new();



#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();



#[used]
#[unsafe(link_section=".requests")]
pub static ENTRY_REQUEST: EntryPointRequest = EntryPointRequest::new(_start);



#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();



#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();



#[unsafe(no_mangle)]
pub  extern "C" fn _start() -> ! {
    
    println!("The kernel is starting!");

    let virt_addr = hhdm_init().expect("The kernel MUST return offset");

    #[cfg(debug_assertions)]
    println!("HHDM is {}", virt_addr);
    
    let mut buddy_system  = BuddyManager::new();
    
    #[cfg(debug_assertions)]
    println!("Buddy manager is {:?}", buddy_system);

    let kernel_alloc = memmap_init(&mut buddy_system, virt_addr);

    let mut page_allocator = PageAllocationMapper::new(virt_addr, buddy_system);

    #[cfg(debug_assertions)]
    println!("Page allocator is {:?}", page_allocator);

    if let Some(fb) = framebuffer_init() &&  let Some(ka) = kernel_alloc  {
        println!("The framebuffer was initilized");

        
        #[cfg(debug_assertions)]
        println!("Framebuffer is {:?}", fb);


        #[cfg(debug_assertions)]
        println!("Bump allocator is {:?}", ka);
 
        let mut bi = BiosInfo::new(fb,  ka, & mut page_allocator);
 

        kernel_main(&mut bi);        
    
    } else {
        println!("The framebuffer or kernel allocator/heap was not initilized");
        panic!();
    } 

    loop {}
}



fn framebuffer_init() -> Option<Framebuffer> {

    if let Some(buff) = FRAMEBUFFER_REQUEST.response() {
        

        let buffer = buff.framebuffers()[0];
        
        let fb = Framebuffer::new(
            buffer.address(),
            buffer.width,
            buffer.height,
            buffer.pitch,
            buffer.red_mask_size, buffer.green_mask_size, buffer.blue_mask_size,
            buffer.red_mask_shift, buffer.green_mask_shift, buffer.blue_mask_shift,
            buffer.bpp / 8
        );

        Some(fb)
    
    } else {
        None
    } 


}



fn memmap_init(alloc: &mut BuddyManager, offset: u64) -> Option<BumpAllocator> {
    
    if let Some(memmap) = MEMMAP.response() {
        
        println!("Initializing memory map entry!");

        
        let entries = memmap.entries();
        
        let mut kernel_alloc = init_kernel_alloc(entries, offset);
        
        for entry in entries {
            if let Some(k_alloc) = kernel_alloc.as_mut() && entry.type_ == MEMMAP_USABLE {

                let mut len = entry.length as usize;
                let mut base = entry.base;

                if k_alloc.start == base as usize {
                    base += KERNEL_HEAP_SIZE as u64;
                    len -= KERNEL_HEAP_SIZE
                }

                if len == 0 {
                    continue;
                }


                #[cfg(debug_assertions)]
                println!("Memmap entry has base {} and length {}", base, len);

                alloc.add_region(
                    base as *mut u8, 
                    len,
                    k_alloc
                );
            }
        }

        kernel_alloc
    
    } else {
        println!("No memory map available!");
        None
    }
}



fn hhdm_init() -> Option<u64>{
    if let Some(resp) = HHDM.response() {
        Some(
            resp.offset
        )
    } else {
        None
    }
}

fn init_kernel_alloc(entries: &[&Entry], offset: u64) -> Option<BumpAllocator> {
    for entry in entries {
        if entry.type_ == MEMMAP_USABLE && entry.length as usize >= KERNEL_HEAP_SIZE {
            return Some(BumpAllocator::new(entry.base as usize + offset as usize, KERNEL_HEAP_SIZE));
        }
    }
    None
}