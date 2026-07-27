use core::marker::PhantomData;

use crate::{arch::enhanced_pci_mechanism::EnhancedPciMechanism, hal::addresses::{PhysicalAddress, VirtualAddress}, println};

pub struct McfgIterator<'a, P: PhysicalAddress> {
    pub data_slice: &'a [u8],
    pub curr_offset: usize,
    pub hhdm: usize,
    pub _phantom: PhantomData<P>
}

impl <'a, P: PhysicalAddress> Iterator for McfgIterator<'a, P> {
    type Item = EnhancedPciMechanism;

    fn next(&mut self) -> Option<Self::Item> {
        
        if (self.curr_offset + 16) > self.data_slice.len() {
            return None;
        }

        let mut pci_bytes = [0u8; 16];

        pci_bytes.copy_from_slice(
            &self.data_slice[ self.curr_offset .. self.curr_offset+16 ]
        );

        self.curr_offset += 16;

        Some(
            parse_into_pci::<P>(pci_bytes, self.hhdm as u64)
        )
    }
}

fn parse_into_pci<P:PhysicalAddress>(bytes: [u8; 16], hhdm: u64) -> EnhancedPciMechanism {

    let bacm = u64::from_le_bytes(
        bytes[0..8].try_into().unwrap()
    ); 

    let bacm = P::from_u64(bacm);

    let virt_addr = bacm.to_virtual(hhdm)
            .expect("ACPI physical address is invalid!");

    assert!(virt_addr.is_higher_half(), "Address is not in Higher-Half!");

    let pci_segment  = u16::from_le_bytes(
        bytes[8..10].try_into().unwrap()
    );
    
    let start_bus    = bytes[10];
    
    let end_bus      = bytes[11];

    let reserved = u32::from_le_bytes(
        bytes[12..16].try_into().unwrap()
    );

    println!("Virt address {} while a physical {}, on the other hand hhdm {}", virt_addr.to_u64(), bacm.to_u64(), hhdm);

    EnhancedPciMechanism::new(virt_addr.to_u64(), pci_segment, start_bus, end_bus, reserved)
}