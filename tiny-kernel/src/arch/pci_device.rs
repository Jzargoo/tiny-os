use core::fmt::{Debug, Display};

use crate::arch::{device_capibilities::PciCapabilities, header_specific_pci::{BridgeDevice, NormalDevice}};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8, 
    pub function: u8,
    pub base: BasicDevice,
    pub header_type: PciHeaderType,
    pub config_address: Option<u64>,                    // if exists -> mmio  
    pub capabilities: Option<PciCapabilities>
}



#[repr(C)]
#[derive(Debug)]
pub struct BasicDevice{
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision: u8,
    pub prog_if: u8,
    pub sub_class: u8,
    pub class: u8,
    pub cache_line_size: u8,
    pub timer: u8,
    pub header_type: u8,
    pub bist: u8
}



#[allow(
#[derive(Debug)]
pub enum PciHeaderType {
    NORMAL(NormalDevice),
    BRIDGE(BridgeDevice),
    UNKNOWN(u8)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Bar {
    Memory32(u32),
    Memory64(u64),
    Io(u32),
}



pub unsafe fn parse_bars<const N: usize>(ptr: *const u8) -> [Option<Bar>; N] {

    let mut bars = [None; N];

    let mut i = 0;

    let mut reg_counter = 0;
    
    while i < N {
        
        let bar_offset = 0x10 + (reg_counter * 4);
        
        let bar_value = unsafe { *(ptr.add(bar_offset) as *const u32) };
        
        if bar_value == 0 {

            i += 1;

            reg_counter +=1;

            continue;

        }

        if (bar_value & 0x01) == 0 {
            // Memory Space
            
            if ((bar_value >> 1) & 0x03) == 2  {
                
                let bar_upper = unsafe { *(ptr.add(bar_offset + 4) as *const u32) };
                
                let addr = ((bar_upper as u64) << 32) | ((bar_value & 0xFFFF_FFF0) as u64);
                
                bars[i] = Some(Bar::Memory64(addr));
                
                reg_counter += 2;

            } else {
            
                let addr = (bar_value & 0xFFFF_FFF0) as u64;
            
                bars[i] = Some(Bar::Memory32(addr as u32));
            
                reg_counter += 1;
            }

        } else {
            
            // I/O Space
            
            let port = bar_value & 0xFFFF_FFFC;
            
            bars[i] = Some(Bar::Io(port));
            
            reg_counter += 1;
        }

        i += 1;
    }

    bars
}



#[allow(dead_code)]
impl PciDevice {
    pub fn new (
        bus: u8,
        slot: u8, 
        function: u8,
        base: BasicDevice,
        header_type: PciHeaderType,
        config_address: Option<u64>,
        capabilities: Option<PciCapabilities>
    ) -> Self {
        
        Self { 
            bus, slot, function,
            header_type,
            base,
            config_address,
            capabilities
        }

    }

    pub unsafe fn msi_enabled(&self) -> bool {

        if let Some(capabilities) = &self.capabilities {
        
            capabilities.msi.is_some()
        
        } else {
            
            false
        
        }

    }
    

}

impl Display for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PciDevice")
        .field("vendor_id", &self.base.vendor_id)
        .field("device_id", &self.base.device_id)
        .finish()
    }
}

#[allow(dead_code)]
impl PciDevice {
    pub fn is_enabled(&self) -> bool {
        ((self.base.command >> 10) & 0x1) == 1
    } 

    pub fn support_capabilities(&self) -> bool {
        ((self.base.status >> 4) & 0x1) == 1
    }
}