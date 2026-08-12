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



#[allow(dead_code)]
#[derive(Debug)]
pub enum PciHeaderType {
    NORMAL(NormalDevice),
    BRIDGE(BridgeDevice),
    UNKNOWN(u8)
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