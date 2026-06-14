use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, memmap::{Entry, MEMMAP_USABLE}, request::{
        EntryPointRequest, FramebufferRequest, HhdmRequest, MemmapRequest, StackSizeRequest
    }
};

use x86_64::VirtAddr;

use crate::{
    hal::{    
        KERNEL_HEAP_SIZE, bios_info::BiosInfo, framebuffer::Framebuffer, kernel_allocator::BumpAllocator, x86_64_page_allocator::BuddyAlloc
    },
    kernel_main,
    logger::LOGGER};


    
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
pub static STACK: StackSizeRequest = StackSizeRequest::new(65536);



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

    LOGGER.lock().write("The kernel is starting...");

    let virt_addr = hhdm_init().expect("The kernel MUST return offset");
    
    let mut alloc = BuddyAlloc::new(virt_addr);

    let kernel_alloc = memmap_init(&mut alloc, virt_addr.as_u64());


    if let Some(fb) = framebuffer_init() &&  let Some(ka) = kernel_alloc  {
        LOGGER.lock().write("The framebuffer was initilized");
 
        let virt_addr = hhdm_init().expect("The kernel MUST return offset"); // The kernel MUST return offset
 
        let mut bi = BiosInfo::new(fb, virt_addr.as_u64(), ka);
        
        kernel_main(&mut bi);        
    
    } else {
        LOGGER.lock().write("The framebuffer or kernel allocator/heap was not initilized");
    } 

    loop {}
}



fn framebuffer_init() -> Option<Framebuffer> {

    if let Some(buff) = FRAMEBUFFER_REQUEST.response() {
        

        let buffer = buff.framebuffers()[0];
        // test_draw_square(buffer);
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
        LOGGER.lock().write("Framebuffer was not initialized. Response is none");
        None
    } 


}



fn memmap_init(alloc: &mut BuddyAlloc, offset: u64) -> Option<BumpAllocator> {
    
    if let Some(memmap) = MEMMAP.response() {
        
        LOGGER.lock().write("Memory map entry!");
        
        let entries = memmap.entries();
        
        let mut kernel_alloc = init_kernel_alloc(entries);
        
        for entry in entries {
            if let Some(k_alloc) = kernel_alloc.as_mut() && entry.type_ == MEMMAP_USABLE {
                
                let mut len = entry.length as usize;
                let mut base = entry.base;

                if k_alloc.start == base as usize {
                    base += KERNEL_HEAP_SIZE as u64;
                    len -= KERNEL_HEAP_SIZE
                }

                alloc.add_region(
                    (base + offset ) as *mut u8, 
                    len,
                    k_alloc);
            }
        }

        kernel_alloc
    
    } else {
        LOGGER.lock().write("No memory map available!");
        None
    }
}



fn hhdm_init() -> Option<VirtAddr>{
    if let Some(resp) = HHDM.response() {
        LOGGER.lock().write("HHDM was initialized!");
        Some(
            VirtAddr::new(resp.offset)
        )
    } else {
        LOGGER.lock().write("HHDM was NOT initialized!");
        None
    }
}

fn init_kernel_alloc(entries: &[&Entry]) -> Option<BumpAllocator> {
    for entry in entries {
        if entry.type_ == MEMMAP_USABLE && entry.length as usize >= KERNEL_HEAP_SIZE {
            return Some(BumpAllocator::new(entry.base as usize, KERNEL_HEAP_SIZE));
        }
    }
    None
}