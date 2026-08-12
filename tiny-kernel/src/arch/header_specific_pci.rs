#[derive(Debug)]
#[allow(dead_code)]
pub struct BridgeDevice {
    pub bars: [Bar; 2],       
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
    pub erom_bar: u32,
    pub bridge_control: u16,
    pub multifunc: bool
}



#[derive(Debug)]
#[repr(C)]
#[allow(dead_code)]
pub struct NormalDevice {
    pub bars: [Bar; 6],
    pub cardbus: u32,
    pub sub_system_id: u16,
    pub sub_system_vendor_id: u16,
    pub exp_rom_base_address: u32,
    pub multifunc: bool
}



#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Bar {
    Memory32(u32),
    Memory64(u64),
    Io(u32),
}

impl Bar {
    fn empty() -> Self{
        Self::Memory32(0)
    }

    fn is_usable (&self) -> bool {
        match self {
            Bar::Io(f) => *f != 0,
            Bar::Memory64(f) => *f != 0,
            Bar::Memory32(f) => *f != 0 
        }
    }
}



pub unsafe fn parse_bars<const N: usize>(first_bar_ptr: *const u8) -> [Bar; N] {

    let mut bars = [Bar::empty(); N];


    let mut reg_counter = 0;
    
    for i in 0..N {
        
        let bar_offset = reg_counter * 4;
    
        let bar_value = unsafe { *(first_bar_ptr.add(bar_offset) as *const u32) };
        
        if bar_value == 0 {

            reg_counter += 1;

            continue;

        }

        if (bar_value & 0x01) == 0 {
            // Memory Space
            
            if ((bar_value >> 1) & 0x03) == 2  {
                
                let bar_upper = unsafe { *(first_bar_ptr.add(bar_offset + 4) as *const u32) };
                
                let addr = ((bar_upper as u64) << 32) | ((bar_value & 0xFFFF_FFF0) as u64);
                
                bars[i] = Bar::Memory64(addr);
                
                reg_counter += 2;

            } else {
            
                let addr = (bar_value & 0xFFFF_FFF0) as u64;
            
                bars[i] = Bar::Memory32(addr as u32);
            
                reg_counter += 1;
            }

        } else {
            
            // I/O Space
            
            let port = bar_value & 0xFFFF_FFFC;
            
            bars[i] = Bar::Io(port);
            
            reg_counter += 1;
        }

    }

    bars
}


impl NormalDevice {
    pub unsafe fn parse(bdf_address: u64, multifunc: bool) -> NormalDevice {
    
        let ptr = bdf_address as *const u8;

        let bars = unsafe{ parse_bars::<6>(ptr.add(0x10)) };
    
    
        NormalDevice { 
            bars, 
        
            sub_system_vendor_id: unsafe { *(ptr.add(0x2C) as *const u16) },
        
            sub_system_id:        unsafe { *(ptr.add(0x2E) as *const u16) },
        
            exp_rom_base_address: unsafe { *(ptr.add(0x30) as *const u32) },

            cardbus:              unsafe { *(ptr.add(0x28) as *const u32) },

            multifunc        
        }
    }
}

impl BridgeDevice {
    pub unsafe fn parse(start_address: u64, multifunc: bool) -> Self {

        
        let bars = unsafe {
            parse_bars::<2>(
                (start_address as *const u8).add(0x10)
            )
        }; 
        
        
        let ptr = start_address as *const u8;

        BridgeDevice { 
            
            bars: bars, 

            primary_bus:                     unsafe { *( ptr.add(0x18) )},
            
            secondary_bus:                   unsafe { *( ptr.add(0x19) )},
            
            subordinate_bus:                 unsafe { *( ptr.add(0x1A) )},
            
            secondary_latency_timer:         unsafe { *( ptr.add(0x1B) )},
            
            io_base:                         unsafe { *( ptr.add(0x1C) )},
            
            io_limit:                        unsafe { *( ptr.add(0x1D) )},
            
            secondary_status:                unsafe { *( ptr.add(0x1E) as *const u16)},
            
            memory_base:                     unsafe { *( ptr.add(0x20) as *const u16)},
            
            memory_limit:                    unsafe { *( ptr.add(0x22) as *const u16)},
            
            prefetchable_memory_base:        unsafe { *( ptr.add(0x24) as *const u16)},
            
            prefetchable_memory_limit:       unsafe { *( ptr.add(0x26) as *const u16)},
            
            prefetchable_memory_base_upper:  unsafe { *( ptr.add(0x28) as *const u32)},
            
            prefetchable_memory_limit_upper: unsafe { *( ptr.add(0x2C) as *const u32)},
            
            io_base_upper:                   unsafe { *( ptr.add(0x30) as *const u16)},
            
            io_limit_upper:                  unsafe { *( ptr.add(0x32) as *const u16)},
            
            erom_bar:                        unsafe { *( ptr.add(0x38) as *const u32)},
            
            bridge_control:                  unsafe { *( ptr.add(0x3E) as *const u16)},

            multifunc
        }
        
    }

}