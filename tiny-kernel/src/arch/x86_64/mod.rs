use x86_64::{PhysAddr, VirtAddr};

use crate::{acpi::{fadt::Fadt, hpet::Hpet, madt::Madt, mcfg::Mcfg, rsdp::{Rsdp, RsdpCommon}, table_registry::{TableRegistry, Tables}, xsdt::Xsdt, xsdt_iter::RxsdtToIter}, arch::{scheduling::schedule, x86_64::interrupts::{VECTOR_INTERRUPT_ALLOCATOR, lapic::APIC_DRIVER, lapic_requests_options::TimerOptions}}, hal::addresses::{PhysicalAddress, VirtualAddress}};

pub mod page_allocator;

pub mod boot;

pub mod interrupts;

impl VirtualAddress for VirtAddr {
    fn to_u64(&self) -> u64 {
        self.as_u64()
    }

    fn is_higher_half(&self) -> bool {
        self.to_u64() >= 0xFFFF_8000_0000_0000
    }

    unsafe fn to_ptr<T>(&self) -> *const T {
        self.as_ptr()
    }
}

impl PhysicalAddress for x86_64::PhysAddr {
    fn to_u64(&self) -> u64 {
        self.as_u64()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn from_u64(addr: u64) -> Self {
        PhysAddr::new(addr)
    }

    type VAddr = VirtAddr;

    fn to_virtual(&self, offset: u64) -> Option<Self::VAddr> {
        Some(
            VirtAddr::new(
                self.as_u64() + offset 
            )
        )
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


pub(self) fn setup_lapic(madt: &Madt<x86_64::PhysAddr>) -> bool{
    madt.set_lapic();

    if let Some(vin) = 
                            VECTOR_INTERRUPT_ALLOCATOR.lock()
                                .set_and_get_free_vector(schedule) {    
        
        let  options = TimerOptions::new(
            1,
            1, 
            vin, 
            0b1
        );

        if let Some(driver) = APIC_DRIVER.get() {
            
            unsafe { driver.setup_timer(options) };

            return true;
        
        }

        false

    } else {
        false
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
    
    let mut fadt = None;
    let mut madt = None;
    let mut hpet = None;
    let mut mcfg = None;

    for i in xsdt.to_iter() {
        
        if i.signature.eq(
            &Tables::get_signature(&Tables::FADT)
        ) {

            fadt = Some( Fadt::new() );

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
        xsdt, fadt, madt, hpet, mcfg
    }

}