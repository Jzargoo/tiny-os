use core::marker::PhantomData;
use core::slice::from_raw_parts;

use crate::acpi::acpi_sdt_header::AcpiSdtHeader;
use crate::acpi::xsdt_iter::{RxsdtToIter, XsdtIter};
use crate::hal::addresses::PhysicalAddress;


#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Xsdt<P: PhysicalAddress>{
    pub sdt: &'static AcpiSdtHeader,
    hh_mem: u64,
    _phantom: PhantomData<P>
}

impl<P: PhysicalAddress> Xsdt<P> {
    pub fn new (sdt: &'static AcpiSdtHeader, hh_mem: u64) -> Self{
        Xsdt::<P>{
            sdt, hh_mem,
            _phantom: PhantomData
        }
    }

    pub fn as_slice (&self) -> &'static [u8] {
        unsafe {
        
            from_raw_parts(self.sdt.get_raw_data_addres() as *const u8, self.sdt.get_data_len())
        
        }

    }
}

impl <'a, P: PhysicalAddress> RxsdtToIter<'a, P> for Xsdt<P> {
    fn to_iter(&self) -> XsdtIter<'a, P> {
        XsdtIter {
            current_offset: 0,
            hh_mem_offset: self.hh_mem,
            slice: self.as_slice(),
            _phantom: PhantomData
        }
    }
}