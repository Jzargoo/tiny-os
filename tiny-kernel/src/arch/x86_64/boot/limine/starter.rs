use crate::{
    acpi::{acpi_sdt_header::AcpiSdtHeader, xsdt::Xsdt}, arch::x86_64::{
        boot::limine::{limine_framebuffer::framebuffer_init, limine_memory::{hhdm_init, lapic_mmio_init, memmap_init, mmio_init, pci_mechanism_init}, limine_requests::{HHDM, REQUESTS_RSDP}}, hlt_loop, interrupts::enable_cpu_interrupts, page_allocator::PageAllocationMapper, parse_acpi_tables, rsdp, setup_lapic
    }, hal::{
        bios_info::BiosInfo, buddy_mem_manager::BuddyManager
    }, kernel_main, println
};




#[unsafe(no_mangle)]
pub  extern "C" fn _start() -> ! {
    
    println!("The kernel is starting!");

    // HHDM INITIALIZATION
    let virt_addr = hhdm_init().expect("The kernel MUST return offset");
    


    // RSDP
    let rsdp = rsdp(
        REQUESTS_RSDP.response().expect("The kernel MUST have rsdp").address as u64
    );

    #[cfg(debug_assertions)]
    unsafe {        
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
        pci_mechanism_init(
            &mut page_allocator.ptr_table,
            &mut page_allocator.buddy_manager, 
            i.end_pci_host_bridge as usize - i.start_pci_host_bridge as usize + 1usize, 
            virt_addr, 
            i.bacm - virt_addr
        );
    }






    // INTERRUPTS INITIALIZATION
    enable_cpu_interrupts();

    if let Some(madt) = &tables.madt {
        let phys_base = madt.set_lapic();

        lapic_mmio_init(
            &mut page_allocator.ptr_table,
            &mut page_allocator.buddy_manager, 
            virt_addr,
            phys_base.as_u64() 
        );

        setup_lapic(madt);
        
    } else {
        panic!("Could not set up lapic table from ACPI specification")
    }


    x86_64::instructions::interrupts::int3();
    
    

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