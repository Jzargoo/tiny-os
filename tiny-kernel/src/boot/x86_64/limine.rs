use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, memmap::MEMMAP_USABLE, request::{
        EntryPointRequest, FramebufferRequest, HhdmRequest, MemmapRequest, StackSizeRequest
    }
};

use x86_64::VirtAddr;

use crate::{
    hal::{    
        GREEN, RED, bios_info::BiosInfo, framebuffer::Framebuffer, x86_64_page_allocator::BuddyAlloc
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

    memmap_init(&mut alloc, virt_addr.as_u64());


    if let Some(fb) = framebuffer_init() {
        LOGGER.lock().write("The framebuffer was initilized");
 
        let virt_addr = hhdm_init().expect("The kernel MUST return offset"); // The kernel MUST return offset
 
        let mut bi = BiosInfo::new(fb, virt_addr.as_u64());
        
        kernel_main(&mut bi) ;        
    
    } else {
        LOGGER.lock().write("The framebuffer was not initilized");
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



fn memmap_init(alloc: &mut BuddyAlloc, offset: u64) {
    if let Some(memmap) = MEMMAP.response() {
        LOGGER.lock().write("Memory map entry!");
        let entries = memmap.entries();
        for entry in entries {
            if entry.type_ == MEMMAP_USABLE {
                alloc.add_region(
                    (offset + entry.base) as *mut u8,
                     entry.length as usize);

                
            }
        }
    } else {
        LOGGER.lock().write("No memory map available!");
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

pub fn test_draw_square(fb: &limine::framebuffer::Framebuffer) {

    let bpp = fb.bpp as usize / 8;
    let pitch = fb.pitch as usize;

    let fb_ptr = fb.address() as *mut u8;

    unsafe {
        for y in 0..200 {
            for x in 0..200 {
                let offset = y * pitch + x * bpp;
                let pixel = fb_ptr.add(offset);

                pixel.add(0).write_volatile(0x00); // B
                pixel.add(1).write_volatile(0x00); // G
                pixel.add(2).write_volatile(0xFF); // R

                if bpp == 4 {
                    pixel.add(3).write_volatile(0x00); // A
                }
            }
        }
    }
}