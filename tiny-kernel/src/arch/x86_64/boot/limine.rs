use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, memmap::{Entry, MEMMAP_USABLE}, request::{
        EntryPointRequest,  FramebufferRequest, HhdmRequest, MemmapRequest, RsdpRequest, StackSizeRequest
    }
};
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB}};

use crate::{
    acpi::{acpi_sdt_header::AcpiSdtHeader, facp::Facp, hpet::Hpet, madt::Madt, mcfg::Mcfg, rsdp::{Rsdp, RsdpCommon}, table_registry::{TableRegistry, Tables}, xsdt::Xsdt, xsdt_iter::RxsdtToIter}, arch::x86_64::{interrupts::{enable_cpu_interrupts}, page_allocator::PageAllocationMapper}, hal::{    
        KERNEL_HEAP_SIZE, bios_info::BiosInfo, buddy_mem_manager::BuddyManager, framebuffer::Framebuffer, kernel_allocator::BumpAllocator
    }, kernel_main, println};


    
#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();



#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".requests")]
pub static REQUESTS_RSDP: RsdpRequest = RsdpRequest::new();




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

    // HHDM INITIALIZATION
    let virt_addr = hhdm_init().expect("The kernel MUST return offset");
    


    // RSDP
    let rsdp = rsdp(
        REQUESTS_RSDP.response().expect("The kernel MUST have rsdp").address as u64
    );

    unsafe {
        #[cfg(debug_assertions)]
        println!("RSDP is {:?}", (*rsdp) );
    }

    let xsdt = unsafe { 
        
            ((*rsdp).xsdt_address + virt_addr) 
                as *const AcpiSdtHeader
        
    }; 

    let xsdt = Xsdt::<x86_64::PhysAddr>::new(
        unsafe {
            xsdt.as_ref()
                .expect("Was expected a correct reference to acpi sdt headers")
        }, 
        virt_addr
    );

    let tables = parse_acpi_tables(xsdt, virt_addr as usize);

    
    
    // INTERRUPTS INITIALIZATION

    enable_cpu_interrupts();


    
    
    x86_64::instructions::interrupts::int3();
    

    #[cfg(debug_assertions)]
    println!("HHDM is {}", virt_addr);
    
    
    
    // BUDDY INITIALIZATION
    let mut buddy_system  = BuddyManager::new();
    
    
    
    #[cfg(debug_assertions)]
    println!("Buddy manager is {:?}", buddy_system);

    
    
    // MEMMAP INIT; FILL THE REGIONS INTO BUDDY; INITIALIZE kernel_alloc for buddy purposes
    let kernel_alloc = memmap_init(&mut buddy_system, virt_addr);



    // PAGE MAPPER INITIALIZATION
    let mut page_allocator = PageAllocationMapper::new(virt_addr, buddy_system);



    #[cfg(debug_assertions)]
    println!("Page allocator is {:?}", page_allocator);


    
    //MMIO INIT
    let mcfg = &tables.mcfg.unwrap();
    for i in mcfg.to_iter() {
        println!("{:?}", i);
        mmio_init(
            &mut page_allocator.ptr_table,
            &mut page_allocator.buddy_manager, 
            i.end_pci_host_bridge as usize - i.start_pci_host_bridge as usize + 1usize, 
            virt_addr, 
            i.bacm - virt_addr
        );
    }


    
    // FRAMEBUFFER INITIALIZATION
    if let Some(fb) = framebuffer_init() &&  let Some(ka) = kernel_alloc  {
        println!("The framebuffer was initilized");

        
        #[cfg(debug_assertions)]
        println!("Framebuffer is {:?}", fb);


        #[cfg(debug_assertions)]
        println!("Bump allocator is {:?}", ka);
 

        // COLLECT ALL INFORMATION INO BIOS INFO STRUCTURE 
        let mut bi = BiosInfo::new(
            fb,
            ka,
            & mut page_allocator,
            tables
        );
 

        // INVOKE MAIN KERNEL FUNCTION
        kernel_main(&mut bi);        
    
    } else {
        println!("The framebuffer or kernel allocator/heap was not initilized");
        panic!();
    } 

    hlt_loop()
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

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

fn rsdp(address: u64) -> *const Rsdp {
    let pointer = address as *const RsdpCommon;

    if unsafe { (*pointer).revision } >= 2 {
        if !is_rsdp_valid(address as *const u8, 36) {
            
            panic!("Rsdp is incorrect! Checksum is wrong");
        }
        address as *const Rsdp       
    } else {
        panic!("We do not support rsdp of the first revision that it provided primarily for 32bit system.")
    }
}

// 36 bits for v2+
// 20 bits for v1,0
pub fn is_rsdp_valid(ptr: *const u8, len: usize) -> bool {
    let mut sum: u8 = 0;
    for i in 0..len {
        unsafe {
            sum = sum.wrapping_add(*ptr.add(i));
        }
    }
    sum == 0
}

pub fn parse_acpi_tables(xsdt: Xsdt<PhysAddr>, hhdm: usize) -> TableRegistry<PhysAddr> {
    
    let mut facp = None;
    let mut madt = None;
    let mut hpet = None;
    let mut mcfg = None;

    for i in xsdt.to_iter() {
        
        if i.signature.eq(
            &Tables::get_signature(&Tables::FACP)
        ) {

            facp = Some( Facp::new() );

        } else if i.signature.eq(
            &Tables::get_signature(&Tables::MCFG)
        ) {

            mcfg = Mcfg::new(i, hhdm)

        } else if i.signature.eq(
            &Tables::get_signature(&Tables::HPET)
        ) {

            hpet = Some( Hpet::new() );

        } else if i.signature.eq(
            &Tables::get_signature(&Tables::MADT)
        ){

            madt = Some( Madt::new(i) );
            
        }

    }

    TableRegistry{
        xsdt, facp, madt, hpet, mcfg
    }

}

pub fn mmio_init<M,A>(
    mapper: &mut M,
    alloc: &mut A,
    bus_count: usize,
    hhdm: u64,
    phys_base: u64,
) where M: Mapper<Size4KiB>, A: FrameAllocator<Size4KiB> {

    let size = bus_count as u64 * 1024 * 1024; 

    let phys_addr = PhysAddr::new(phys_base);
    let virt_addr = VirtAddr::new(hhdm + phys_base);

    let start_page: Page<Size4KiB> = Page::containing_address(virt_addr);
    let end_page: Page<Size4KiB> = Page::containing_address(virt_addr + size);

    let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::NO_CACHE;
    
    for page in Page::range_inclusive(start_page, end_page) {
        let frame = PhysFrame::containing_address(
            phys_addr + (page.start_address() - virt_addr)
        );
        
        if let Ok(_existing) = mapper.translate_page(page) {
            
            unsafe {
            
                if let Ok(flush) = mapper.update_flags(page, flags) {
                    flush.flush();
                }
            
            }

        } else {
        
            unsafe {
            
                mapper
                    .map_to(page, frame, flags, alloc)
                    .expect("Failed to map ECAM page")
                    .flush(); 
            
            }

        }

    }

}