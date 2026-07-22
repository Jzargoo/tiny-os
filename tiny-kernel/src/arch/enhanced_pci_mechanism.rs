#[repr(C, packed)]
#[derive(Debug)]
pub struct EnhancedPciMechanism{
    bacm: u64,
    bus_segment_number: u16,
    start_pci_host_bridge: u8,
    end_pci_host_bridge: u8,
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
}