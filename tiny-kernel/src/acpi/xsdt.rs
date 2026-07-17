use core::ptr::read_unaligned;

use crate::acpi::acpi_sdt_header::AcpiSdtHeader;
#[cfg(debug_assertions)]
use crate::println;

#[repr(C, packed)]
#[derive(Clone, Copy)]

pub struct XsdtInternals{
    pub acpi_sdt_header: AcpiSdtHeader,
    pub other_tables_addres: u64
}

pub struct Xsdt{
    pub xsdt_internals: *const XsdtInternals,
    pub hh_mem: u64
}

impl Xsdt {


    pub unsafe fn entries(&self) -> usize {
        let len = unsafe{ 
            (
                read_unaligned(self.xsdt_internals)
            )
                .acpi_sdt_header.len as usize
        };

        (len - 36usize) / 8usize
    }


    pub unsafe fn get_entry(&self, req_table: Tables) -> Option<AcpiSdtHeader> {
        let count = unsafe { self.entries() };

        for i in 0..count {
            let entry = unsafe { self.get_entry_by_index(i) };

            if entry.signature.eq(&req_table.get_signature()) {
                return Some(entry);
            }
        }

        #[cfg(debug_assertions)]
        println!("Entry {:?} was not found with signature {:?}", req_table, req_table.get_signature());

        None
    }


    pub unsafe fn get_entry_by_index(&self, index: usize) -> AcpiSdtHeader {
    
        if index >= unsafe { self.entries() } {
            panic!("XSDT index out of bounds");
        }

        let xsdt_base = self.xsdt_internals as *const u8;

        let offset = 36 + (index * 8);

        let curr_el_pointer = unsafe { xsdt_base.add(offset) } as *const u64;

        let target_table_phys = unsafe { core::ptr::read_unaligned(curr_el_pointer) };
        
        let target_table_virt = (target_table_phys + self.hh_mem) as *const AcpiSdtHeader;

        unsafe { core::ptr::read_unaligned(target_table_virt) }
    }


    pub fn read_self_headers(&self) -> AcpiSdtHeader{
        unsafe { 
            let header_ptr = self.xsdt_internals as *const AcpiSdtHeader;

            read_unaligned(header_ptr)
        }
    }   
    
}


#[derive(Debug)]
pub enum Tables {
    XSDT,
    MCFG
}


impl Tables {
    pub fn get_signature(&self) -> [u8; 4] {
        match self {
            Tables::XSDT => [b'X',b'S',b'D',b'T'],
            Tables::MCFG => [b'M',b'C',b'F',b'G']
        }
    }
}