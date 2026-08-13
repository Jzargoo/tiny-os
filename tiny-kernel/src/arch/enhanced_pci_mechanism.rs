use alloc::vec::Vec;

use crate::{arch::{device_capibilities::PciCapabilities, header_specific_pci::{BridgeDevice, NormalDevice}, pci_device::{BasicDevice, PciHeaderType}}, println};

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

// parse pci device 
fn parse_pci_device(start_address: u64, bus: u8  , slot: u8, function: u8) -> Option<PciDevice> {

    let ptr = start_address as *const u8;

    unsafe {
    
        let basic = ( ptr as *const BasicDevice ).read();

        if basic.vendor_id == 0xFFFF {
            return None;
        }

        
        println!("{}/{}",
            basic.status, *(start_address as *const u16).add(3)
        );
        
        let cap_pointer = if (basic.status & (1 << 4)) != 0 {
            *ptr.add(0x34)
        } else {
            0
        };

        let header_type_val = basic.header_type & 0x7F;

        let multifunc = (header_type_val >> 7) != 0;

        let header_type = match header_type_val {
            
            0x0 => PciHeaderType::NORMAL(
                NormalDevice::parse(start_address, multifunc)
            ),
            
            0x1 => PciHeaderType::BRIDGE(
                BridgeDevice::parse(start_address, multifunc)
            ),

            _   => PciHeaderType::UNKNOWN(header_type_val) 
        
        };

        let mut capabilities = None;
        
        if cap_pointer != 0 {
            
            capabilities = Some(
                PciCapabilities::parse(start_address, cap_pointer)
            );

        }

        Some(
            PciDevice::new(
                bus, slot, function,
                basic, 
                header_type, 
                Some(start_address),
                capabilities
            )
        )

    }
}



// get address by base and identifiers in the bus
pub fn get_bdf_address(virt_base: u64, bus: u8, dev: u8, func: u8) -> u64 {
    let bus_offset = bus as u64;
    let dev_offset = (dev & 0x1F) as u64;   // 0..31
    let func_offset = (func & 0x07) as u64; // 0..7

    let offset = (bus_offset << 20) | (dev_offset << 15) | (func_offset << 12);
    virt_base + offset
}

   