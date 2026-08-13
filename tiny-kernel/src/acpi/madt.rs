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

    pub fn get_lapic_addr(&self) -> u32{
        let raw_addr = self.sdt.get_raw_data_addres(0);

        unsafe { 
            *(raw_addr as *const u32)
        }
        
    }
}