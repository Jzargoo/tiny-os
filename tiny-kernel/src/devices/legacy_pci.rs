use core::arch::asm;

use alloc::vec::{self, Vec};

use crate::{hal::pci_device::{Bar, BridgeDevice, NormalDevice, PciDevice, PciHeaderType}, println};

const PCI_CONFIG_DATA: u16 = 0xCFC;
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;

fn read_bus(bus: u8, slot: u8, func: u8, offset: u8) -> u32{
    
    let address = ((bus as u32 & 0xFF) << 16) 
                | ((slot as u32 & 0x1F) << 11) 
                | ((func as u32 & 0x07) << 8) 
                | ((offset as u32 & 0xFC)) 
                | 0x80000000;

    outportl(address, PCI_CONFIG_ADDRESS);

    inportl(PCI_CONFIG_DATA)
}

pub fn is_device_exist( bus: u8, slot: u8, func: u8) -> bool {
    read_bus(bus, slot, func, 0) != 0xFFFFFFFF
}


pub fn is_multi_func(bus: u8, slot: u8) -> bool {
    (read_bus(bus, slot, 0, 0) & 0x80) != 0
}

fn outportl(address: u32, address_bus: u16) { 
    unsafe { 
        asm!(
            "out dx, eax",
            in("dx") address_bus,
            in("eax") address,
            options(nomem, preserves_flags, nostack)
        );
    }
} 

fn inportl(address_bus: u16) -> u32{
    let mut value = 0;

    unsafe {
        asm!(
            "in eax, dx",
            out("eax") value,
            in("dx") address_bus,
            options(nomem, preserves_flags, nostack)
        )
    }

    value
}

pub fn init_configuration(bus: u8, slot: u8, func: u8) -> Option<PciDevice> {
    
    let ids = read_bus(bus, slot, func, 0);

    let device_id: u16 = (ids >> 16) as u16;
    let vendor_id: u16 = (ids & 0xFFFF) as u16;
    
    
    let register_1 = read_bus(bus, slot, func, 4);

    let status: u16 =  (register_1 >> 16) as u16;
    let command: u16 = (register_1 & 0xFFFF) as u16;


    let register_2 = read_bus(bus, slot, func, 8);
    
    let class = (register_2 >> 24) as u8;
    let sub = ((register_2 >> 16) & 0xFF) as u8;
    let revision = (register_2 & 0xFF) as u8;
    let prog_if = ((register_2 >> 8) & 0xFF) as u8;


    let register_3 = read_bus(bus, slot, func, 12);

    let bist = (register_3 >> 24) as u8;
    let header_type = ((register_3 >> 16) & 0xFF) as u8;
    let timer = ((register_3 >> 8) & 0xFF) as u8;
    let cache_line_size = (register_3  & 0xFF) as u8;

    if header_type == 2 {
        println!("[WARN]: cannot support devices with header type 2...");
        return None;
    }
    
    let mut pci_header_type = PciHeaderType::UNKNOWN;

    let mut start_offset = 0x10;

    if header_type == 0 {

        let mut bars = [None,None,None,None,None,None];

        for i in 0..bars.len() {

            let bar_and_is_long = bar_init(bus, slot, func, start_offset + (4 * i) );

            if bar_and_is_long.1 {
                start_offset += 4;
            }

            bars[i] = Some(bar_and_is_long.0);

        }

        pci_header_type = PciHeaderType::NORMAL(
            NormalDevice{
                bars
            }
        )
    } 
    /*
    
    else if header_type == 1 {
        
        let mut bars = [None,None];

        for i in 0..bars.len() {
            let bar_and_is_long = bar_init(bus, slot, func, start_offset + (4 * i) );

            if bar_and_is_long.1 {
                start_offset += 4;
            }

            bars[i] = Some(bar_and_is_long.0);

        }

        pci_header_type = PciHeaderType::BRIDGE(
            BridgeDevice{
                bars
            }
        )
    }
    */
    
    Some (
        PciDevice::new(
            vendor_id, 
            device_id, 
            status, 
            command, 
            class, sub, 
            bus, slot, func,
            revision, 
            prog_if,
            bist, 
            pci_header_type, 
            timer, 
            cache_line_size
    )
)
}


fn bar_init(bus: u8, slot: u8, func: u8, offset: usize) -> (Bar,bool) {

    let bar = read_bus(bus, slot, func, offset as u8);
        
    if (bar & 0x1) == 0 {
            
        let mut address= (bar & (!0x7)) as u64;
        
        let is_long = (bar & 0x7) == 0b100;
        
        if is_long {
            let higher_half= read_bus(bus, slot, func, (offset + 4) as u8);

            address = (
                ( ( higher_half as u64 ) << 32) | 
                ( address & 0xFFFFFFFC  ) 
            ) as u64;
            
        }
        
        (
            Bar {
                address,
                is_port: false
            },
            is_long
        )
        
    } else {
        (
            Bar{
                address: (bar & (!0x7)) as u64,
                is_port: true
            },
            false
        )
    }
}
    


    
pub fn scan_pci() -> Vec<PciDevice> {
    let mut devices = Vec::new();

    for bus in 0..=10 {
        
        for slot in 0..32{  
    
            if !is_device_exist(bus, slot, 0) {
                continue;
            } else if !is_milti_func(bus, slot) {
                devices.push(
                    init_configuration(bus, slot, 0)
                ); 
                continue;
            }

            for func in 0..7 {
                if is_device_exist(bus, slot, func) {
                    devices.push(
                        init_configuration(bus, slot, func)
                    );
                }
            }
    
        }

    }

    devices
}