use core::marker::PhantomData;

use crate::{acpi::acpi_sdt_header::AcpiSdtHeader, hal::addresses::PhysicalAddress};

pub struct Madt<P: PhysicalAddress>{
    sdt: &'static AcpiSdtHeader,   
    _phantom: PhantomData<P>
}


impl <P:PhysicalAddress> Madt<P> {
    pub fn new(sdt: &'static AcpiSdtHeader) -> Self{
        Madt { 
            sdt,
            _phantom: PhantomData 
        }
    }

    pub fn get_data_addr(&self) -> u64{

        let raw_addr = self.sdt.get_raw_data_addres(0);

        unsafe { 
            *(raw_addr as *const u64)
        }
        
    }
}

#[cfg(target_arch="x86_64")]
use crate::arch::x86_64::interrupts::lapic::{APIC_DRIVER, ApicDriver};

#[cfg(target_arch="x86_64")]
use x86_64::VirtAddr;

#[cfg(target_arch="x86_64")]
impl <P:PhysicalAddress> Madt<P>{
    
    pub fn set_lapic(&self){

        *APIC_DRIVER.lock() = 
            unsafe { 
                Some (
                    ApicDriver::new(
                        VirtAddr::new(self.get_data_addr())
                    )
                )
            }

    }

}