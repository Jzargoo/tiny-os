use alloc::vec::Vec;

use crate::{arch::{device_capibilities::PciCapabilities, header_specific_pci::NormalDevice, pci_device::{BasicDevice, PciHeaderType, parse_bars}}, println};

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
    
        let basic = *( ptr as *const BasicDevice );
        
        if basic.vendor_id == 0xFFFF {
            return None;
        }

        let cap_pointer = if (basic.status & (1 << 4)) != 0 {
            *ptr.add(0x34)
        } else {
            0
        };

        let header_type_val = basic.header_type & 0x7F;

        let multifunc = (header_type_val >> 7) != 0;

        let header_type = match header_type_val {
            
            0x0 => PciHeaderType::NORMAL(
                parse_normal_header(start_address, multifunc)
            ),
            
            0x1 => PciHeaderType::BRIDGE(
                parse_bridge_header(start_address, multifunc)
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



// parse header for device 0x0
unsafe fn parse_normal_header(bdf_address: u64, multifunc: bool) -> NormalDevice {
    
    let ptr = bdf_address as *const u8;

    let bars = unsafe{ parse_bars::<6>(ptr.add(0x10)) };
    
    
    NormalDevice { 
        bars, 
        
        sub_system_vendor_id: unsafe{ *(ptr.add(0x2C) as *const u16) },
        
        sub_system_id:        unsafe{ *(ptr.add(0x2E) as *const u16) },
        
        exp_rom_base_address: unsafe{ *(ptr.add(0x30) as *const u32) },
        
        interrupt_line:       unsafe{ *ptr.add(0x3C) },
        
        interrupt_pin:        unsafe{ *ptr.add(0x3D) },
        
        multifunc,  
    }
}

unsafe fn parse_bridge_header (bdf_address: u64, multifunc: bool) -> BridgeDevice{

    let ptr = bdf_address as *const u8;

    BridgeDevice { 
        
        bars:                            unsafe { parse_bars::<2>(ptr) }, 
        
        secondary_latency_timer:         unsafe { *(ptr.add(0x18)) }, 
        
        subordinate_bus:                 unsafe { *(ptr.add(0x19)) }, 
        
        secondary_bus:                   unsafe { *(ptr.add(0x1A)) }, 
        
        primary_bus:                     unsafe { *(ptr.add(0x1B)) }, 
        
        secondary_status:                unsafe { *(ptr.add(0x1C) as *const u16) },

        io_limit:                        unsafe { *(ptr.add(0x1E)) }, 
        
        io_base:                         unsafe { *(ptr.add(0x1F)) }, 
        
        memory_limit:                    unsafe { *(ptr.add(0x20) as *const u16) }, 
        
        memory_base:                     unsafe { *(ptr.add(0x22) as *const u16) }, 
        
        prefetchable_memory_limit:       unsafe { *(ptr.add(0x24) as *const u16) }, 
        
        prefetchable_memory_base:        unsafe { *(ptr.add(0x26) as *const u16) },
        
        prefetchable_memory_limit_upper: unsafe { *(ptr.add(0x28) as *const u32) }, 
        
        prefetchable_memory_base_upper:  unsafe { *(ptr.add(0x2C) as *const u32) },
        
        io_limit_upper:                  unsafe { *(ptr.add(0x30) as *const u16) }, 
        
        io_base_upper:                   unsafe { *(ptr.add(0x32) as *const u16) },
        
        cap_pointer:                     unsafe { *(ptr.add(0x37)) }, 
        
        erom_bar:                        unsafe { *(ptr.add(0x38) as *const u32) },

        bridge_control:                  unsafe { *(ptr.add(0x3C) as *const u16) }, 
        
        interrupt_pin:                   unsafe { *(ptr.add(0x3E)) }, 
        
        interrupt_line:                  unsafe { *(ptr.add(0x3F)) }, 
        
        multifunc 
    }
}
   