use alloc::vec::Vec;

use crate::{arch::pci_device::PciHeaderType, println};

use super::pci_device::PciDevice;

#[repr(C, packed)]
#[derive(Debug)]
pub struct EnhancedPciMechanism{
    pub bacm: u64,
    bus_segment_number: u16,
    pub start_pci_host_bridge: u8,
    pub end_pci_host_bridge: u8,
    reserved: u32
}

impl EnhancedPciMechanism {
    pub fn new (
        bacm: u64,
        bus_segment_number: u16,
        start_pci_host_bridge: u8,
        end_pci_host_bridge: u8,
        reserved: u32
    ) -> Self{
        EnhancedPciMechanism { 
            bacm,
            bus_segment_number, 
            start_pci_host_bridge, end_pci_host_bridge, 
            reserved 
        }
    }

    pub fn collect_devices(&self) -> Vec<PciDevice>{
        
        let mut collected = Vec::new();

        println!("Starting collecting devices!");
        
        for i in 0..32 {
            
            let start_address = get_bdf_address(self.bacm, self.start_pci_host_bridge,  i, 0);
            
            #[cfg(debug_assertions)]
            println!("Creating a device on the {} slot. Start address {}", i, start_address);

            let pci = 
                parse_pci_device(start_address, self.start_pci_host_bridge, i as u8, 0u8);

                
            if pci.is_some() {

                let pci = pci.unwrap();

                #[cfg(debug_assertions)]
                println!("Parsed a device {:?}", pci);
                    
                collected.push(pci);
            }

        }

        collected
    }
    
}


// Parses 4 registers from the pci devices!
fn parse_pci_device(start_address: u64, bus: u8  , slot: u8, function: u8) -> Option<PciDevice> {

    let ptr = start_address as *const u8;

    unsafe {
    
        let vendor_id = *(ptr.add(0x00) as *const u16);
        let device_id = *(ptr.add(0x02) as *const u16);

        if vendor_id == 0xFFFF {
            return None;
        }

        let command = *(ptr.add(0x04) as *const u16);
        let status  = *(ptr.add(0x06) as *const u16);

        let revision  = *ptr.add(0x08);
        let prog_if   = *ptr.add(0x09);
        let sub_class = *ptr.add(0x0A);
        let class     = *ptr.add(0x0B);

        let cache_line_size = *ptr.add(0x0C);
        let timer           = *ptr.add(0x0D);
        let raw_header_type = *ptr.add(0x0E);
        let bist            = *ptr.add(0x0F);

        let cap_pointer = if (status & (1 << 4)) != 0 {
            *ptr.add(0x34)
        } else {
            0
        };

        let header_type_val = raw_header_type & 0x7F;

        let header_type = PciHeaderType::UNKNOWN(header_type_val);
        
        Some(
            PciDevice::new(vendor_id, device_id, status, command, class, sub_class, bus, slot, function, revision, prog_if, bist, header_type, timer, cache_line_size, cap_pointer)
        )
    }
}

pub fn get_bdf_address(virt_base: u64, bus: u8, dev: u8, func: u8) -> u64 {
    let bus_offset = bus as u64;
    let dev_offset = (dev & 0x1F) as u64;   // 0..31
    let func_offset = (func & 0x07) as u64; // 0..7

    let offset = (bus_offset << 20) | (dev_offset << 15) | (func_offset << 12);
    virt_base + offset
}