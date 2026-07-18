use crate::{acpi::acpi_sdt_header::AcpiSdtHeader, hal::addresses::{PhysicalAddress, VirtualAddress}};

pub struct XsdtIter<'a, P: PhysicalAddress> {
    pub slice: &'a [u8],
    pub current_offset: usize,
    pub hh_mem_offset: u64,
    pub _phantom: core::marker::PhantomData<P>,
}

pub trait RxsdtToIter<'a, P: PhysicalAddress> {
    fn to_iter(&self) -> XsdtIter<'a, P>;
}

impl<'a, P: PhysicalAddress> Iterator for XsdtIter<'a, P> {

    type Item = &'static AcpiSdtHeader;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current_offset + 8 > self.slice.len() {
            return None;
        }

        let mut bytes = [0u8; 8];
        
        bytes.copy_from_slice(&self.slice[self.current_offset..self.current_offset + 8]);

        let raw_addr = u64::from_le_bytes(bytes);
        
        self.current_offset += 8;

        let phys_addr = P::from_u64(raw_addr); 
        
        let virt_addr = phys_addr.to_virtual(self.hh_mem_offset)
            .expect("ACPI physical address is invalid!");

        assert!(virt_addr.is_higher_half(), "Address is not in Higher-Half!");

        unsafe { Some(&*virt_addr.to_ptr::<AcpiSdtHeader>()) }
    }
}