use alloc::vec;


#[derive(Debug)]
pub struct PciCapabilities {
    pub power_management: Option<u8>,
    pub agp: Option<u8>,
    pub vital_product_data: Option<u8>,
    pub slot_identification: Option<u8>,
    pub msi: Option<u8>,
    pub cci: Option<u8>, // CompactPCI Hot Swap
    pub pci_x: Option<u8>,
    pub hypertransport: Option<u8>,
    pub vendor_specific: Option<u8>,
    pub debug_port: Option<u8>,
    pub compactpci_central_resource: Option<u8>,
    pub pci_hot_plug: Option<u8>,
    pub pci_bridge_subsystem_vendor_id: Option<u8>,
    pub agp_8x: Option<u8>,
    pub pcix_2: Option<u8>,
    pub msi_x: Option<u8>,
    pub sata_data_index: Option<u8>,
    pub advanced_features: Option<u8>,
    pub enhanced_allocation: Option<u8>,
    pub flattening_portal_bridge: Option<u8>,
}


#[repr(C)]
#[derive(Debug)]
pub struct MsiCapability {
    pub cap_id: u8,          
    pub next_ptr: u8,        
    pub message_control: u16,
    pub message_address_low: u32
}

impl MsiCapability {

    pub fn is_64bit (&self)-> bool {
        (self.message_control & 0b1000000) != 0
    }

    pub unsafe fn write_vn(&self, vector: u8) {

        let message_data = if self.is_64bit() {
            let ptr = (self as *const MsiCapability) as *const u8;
            
            unsafe { 
                ptr.add(0xC)
            }

        } else {
            
            let ptr = (self as *const MsiCapability) as *const u8;
            
            unsafe { 
                ptr.add(0x8)
            }
            
        };

        let message = MsiCapability::create_msi_message_data(vector);

        unsafe {
            * ( message_data as *mut u16 ) = message; 
        }

    }

    fn create_msi_message_data(vector: u8) -> u16{
        
        let vector_bits = (vector as u16) &0xff;

        let delivery_mode = 0b001 << 8;

        let trigger_mode = 0b0 << 14;

        vector_bits | delivery_mode | trigger_mode
    }

    
    unsafe fn get_64_addres (&self) -> Option<u64> {
        let low_ptr = (&self.message_address_low) as *const u32;

        if !self.is_64bit() {
            None
        } else {

            let high_address = unsafe { 
                *(low_ptr.add(1)) 
            };

            let final_address = ( high_address as u64 ) << 32 + (self.message_address_low as u64);
            
            Some(final_address)
        }

    }
    
}


impl PciCapabilities {
    pub fn empty() -> Self {
        PciCapabilities { 
            power_management: None, 
            agp: None, agp_8x: None, 
            vital_product_data: None, sata_data_index: None,
            slot_identification: None, 
            msi: None, msi_x: None, 
            cci: None, 
            hypertransport: None, 
            vendor_specific: None, debug_port: None, 
            compactpci_central_resource: None, 
            pci_hot_plug: None, pci_x: None, pci_bridge_subsystem_vendor_id: None, pcix_2: None,  
            advanced_features: None, 
            enhanced_allocation: None, 
            flattening_portal_bridge: None 
        }
    }

    pub fn parse(start_address: u64, offset: u8) -> Self {

        let mut curr_offset = offset;
    
        let mut cap_ptr= (start_address + curr_offset as u64) as *const u8;
        
        let mut capabilities = PciCapabilities::empty();

        while curr_offset != 0 {

            let cap_id = unsafe { *cap_ptr };
        
            if cap_id == 0x5 {
                capabilities.msi = Some(curr_offset)
            }    
        
            curr_offset = unsafe { *cap_ptr.add(0x1) }; 
        
            cap_ptr = (start_address + curr_offset as u64) as *const u8;
        }

        capabilities
    }
    
}