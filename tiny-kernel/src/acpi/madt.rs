use core::{marker::PhantomData, slice::from_raw_parts};

use crate::{acpi::{acpi_sdt_header::AcpiSdtHeader, madt_entries::EntryType::EntryType5, madt_iter::MadtIterator}, hal::addresses::PhysicalAddress, println};

#[derive(Debug)]
pub struct Madt<P: PhysicalAddress>{
    sdt: &'static AcpiSdtHeader,   
    hhdm: usize,
    _phantom: PhantomData<P>
}


impl <P:PhysicalAddress> Madt<P> {
    pub fn new(sdt: &'static AcpiSdtHeader, hhdm: usize) -> Self{
        Madt { 
            sdt,
            hhdm,
            _phantom: PhantomData 
        }
    }

    pub fn as_slice (&self) -> &'static [u8] {
     
        unsafe {
        
            from_raw_parts(
                self.sdt.get_raw_data_addres(8) as *const u8, 
                self.sdt.get_data_len(8)
            )
        
        }

    }

    
    pub fn get_lapic_addres(&self) -> u64{

        println!("iterating madt...");
        
        for i in self.to_iter(){

            println!(" entry is {:?}", i);

            let addr = match i.entry_specific {
                EntryType5(entry_address)  => Some( 
                    entry_address.lapic_address + self.hhdm as u64
                ),
                
                _ => None
            };

            if addr.is_some(){
                return addr.unwrap() + self.hhdm as u64;
            }

        }

        self.get_32_lapic_addr() as u64 + self.hhdm as u64

    }

    fn get_32_lapic_addr(&self) -> u32 {

        let raw_addr = self.sdt.get_raw_data_addres(0);
        
        unsafe { *(raw_addr as *const u32) }
    }
}

impl <'a, P:PhysicalAddress> Madt<P>{
    
    pub fn to_iter(&self) -> MadtIterator<'a, P>{
    
        MadtIterator { 
            data_slice: self.as_slice(), 
            curr_offset: 0, 
            phantom_data: PhantomData::<P> 
        }

    }

}

#[cfg(target_arch="x86_64")]
use crate::arch::x86_64::interrupts::lapic::{APIC_DRIVER, ApicDriver};

#[cfg(target_arch="x86_64")]
use x86_64::VirtAddr;

impl <P:PhysicalAddress> Madt<P>{
    
    pub fn set_lapic(&self) {

        let data_addr = self.get_lapic_addres();

        println!("{} is lapic addr", data_addr);

        APIC_DRIVER.call_once( || {

            unsafe { 
                ApicDriver::new(
                    VirtAddr::new(data_addr)
                )
            }
    
            }
        );

    }
}