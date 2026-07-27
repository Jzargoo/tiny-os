use core::fmt::Debug;

#[allow(dead_code)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub status: u16,
    pub command: u16,
    pub class: u8,
    pub sub_class: u8,
    pub bus: u8,
    pub slot: u8, 
    pub function: u8,
    pub revision: u8,
    pub prog_if: u8,
    pub bist: u8,
    pub header_type: PciHeaderType,
    pub timer: u8,
    pub cache_line_size: u8,
    pub cap_pointer: u8,
}

#[allow(dead_code)]
pub enum PciHeaderType {
    NORMAL(NormalDevice),
    BRIDGE(BridgeDevice),
    UNKNOWN(u8)
}

#[allow(dead_code)]
pub struct NormalDevice {
    pub bars: [Option<Bar>; 6],
    pub sub_system_id: u16,
    pub sub_system_vendor_id: u16,
    pub exp_rom_base_address: u32,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
}

#[allow(dead_code)]#[allow(dead_code)]
pub struct BridgeDevice {
    pub bars: [Option<Bar>; 2],       
    pub primary_bus: u8,             
    pub secondary_bus: u8,           
    pub subordinate_bus: u8,         
    pub secondary_latency_timer: u8,
    pub io_base: u8,
    pub io_limit: u8,
    pub secondary_status: u16,
    pub memory_base: u16,            
    pub memory_limit: u16,
    pub prefetchable_memory_base: u16,
    pub prefetchable_memory_limit: u16,
    pub cap_pointer: u8, 
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub bridge_control: u16,
}

#[allow(dead_code)]
pub struct Bar {
    pub address: u64, 
    pub is_port: bool
}

#[allow(dead_code)]
impl PciDevice {
    pub fn new (
        vendor_id: u16,
        device_id: u16,
        status: u16,
        command: u16,
        class: u8,
        sub_class: u8,
        bus: u8,
        slot: u8, 
        function: u8,
        revision: u8,
        prog_if: u8,
        bist: u8,
        header_type: PciHeaderType,
        timer: u8,
        cache_line_size: u8,
        cap_pointer: u8
    ) -> Self {
        Self { 
            vendor_id,
            device_id, 
            status, 
            command, 
            class, sub_class,
            bus, slot, function,
            revision, 
            prog_if, 
            bist, 
            header_type, 
            timer, 
            cache_line_size,
            cap_pointer 
        }
    }
}

impl Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PciDevice").field("vendor_id", &self.vendor_id).field("device_id", &self.device_id).finish()
    }
}

#[allow(dead_code)]
impl PciDevice {
    pub fn is_enabled(&self) -> bool {
        ((self.command >> 10) & 0x1) == 1
    } 

    pub fn support_capabilities(&self) -> bool {
        ((self.status >> 4) & 0x1) == 1
    }
}