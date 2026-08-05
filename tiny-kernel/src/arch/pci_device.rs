use core::fmt::{Debug, Display};

#[derive(Debug)]
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
#[derive(Debug)]
pub enum PciHeaderType {
    NORMAL(NormalDevice),
    BRIDGE(BridgeDevice),
    UNKNOWN(u8)
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

impl Display for PciDevice {
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