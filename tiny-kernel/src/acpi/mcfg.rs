use core::{marker::PhantomData, slice::from_raw_parts};

use crate::{acpi::{acpi_sdt_header::AcpiSdtHeader, mcfg_iter::McfgIterator, table_registry::Tables}, hal::addresses::PhysicalAddress};

#[derive(Clone, Copy)]
pub struct Mcfg<P: PhysicalAddress> {
    sdt: &'static AcpiSdtHeader,
    _phantom: PhantomData<P>,
    hhdm: usize,
}



impl <P:PhysicalAddress> Mcfg<P> {
    pub fn new(acpi_sdt_header: &'static AcpiSdtHeader, hhdm: usize) -> Option<Self>{
        
        if acpi_sdt_header.signature != Tables::get_signature(&Tables::MCFG) {
        
            None
        
        } else {   
            
            Some(
                Mcfg { 
                    sdt: acpi_sdt_header, 
                    hhdm,
                    _phantom: PhantomData 
                }
            )

        }

    }

    fn as_slice<'a>(&'a self) -> &'a [u8]{
        unsafe{
            from_raw_parts(
                self.sdt.get_raw_data_addres(8) as *const u8,
                self.sdt.get_data_len(8)
            )
        }
    }

    pub fn to_iter<'a>(&'a self) -> McfgIterator<'a, P>{
        McfgIterator{
            data_slice: self.as_slice(),
            curr_offset: 0,
            hhdm: self.hhdm,
            _phantom: PhantomData
        }
    }
}