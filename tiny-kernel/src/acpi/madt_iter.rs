use core::marker::PhantomData;

use crate::{acpi::madt_entries::EntryType, hal::addresses::PhysicalAddress};

pub struct MadtIterator<'a, P: PhysicalAddress>{
    pub data_slice: &'a [u8],
    pub curr_offset: usize,
    pub phantom_data: PhantomData<P>,
    pub hhdm: usize
}

pub struct Entry {
    entry_type: u8,
    record_len: u8,
    pub entry_specific: EntryType 
}

impl <'a, P: PhysicalAddress>Iterator for MadtIterator<'a, P> {
    
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.curr_offset + 2 > self.data_slice.len() {
            None
        } else {

            let entry_type = self.data_slice[self.curr_offset];

            let len = self.data_slice[self.curr_offset + 1];     
            
            let next_offset = self.curr_offset + len as usize;
            
            if len <= 2 {
                return None;
            } else if next_offset > self.data_slice.len() {
                return None;
            }

            let payload = &self.data_slice[self.curr_offset + 2 .. next_offset];

            let entry_specific = EntryType::parse(entry_type, payload);

            self.curr_offset += next_offset;

            Some(
                Entry { 
                    entry_type: entry_type,
                    record_len: len,
                    entry_specific 
                }
            )
        }

    }
}