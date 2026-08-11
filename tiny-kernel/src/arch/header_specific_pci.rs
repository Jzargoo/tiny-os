use crate::arch::pci_device::Bar;

#[derive(Debug)]
#[allow(dead_code)]
pub struct BridgeDevice {
    pub bars: [Option<Bar>; 2],       
    pub primary_bus: u8,             
    pub secondary_bus: u8,           
    pub subordinate_bus: u8,         
    pub secondary_latency_timer: u8,
    pub io_base: u8,
    pub io_limit: u8,
    pub io_base_upper: u16,
    pub io_limit_upper: u16,
    pub secondary_status: u16,
    pub memory_base: u16,            
    pub memory_limit: u16,
    pub prefetchable_memory_base: u16,
    pub prefetchable_memory_limit: u16,
    pub prefetchable_memory_base_upper: u32,
    pub prefetchable_memory_limit_upper: u32,
    pub cap_pointer: u8, 
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub erom_bar: u32,
    pub bridge_control: u16,
    pub multifunc: bool
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct NormalDevice {
    pub bars: [Option<Bar>; 6],
    pub sub_system_id: u16,
    pub sub_system_vendor_id: u16,
    pub exp_rom_base_address: u32,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub multifunc: bool
}