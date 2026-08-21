use core::{arch::naked_asm, sync::atomic::Ordering};

use crate::arch::x86_64::interrupts::interrupt_funcs::IDT;


pub(super) const VECTOR_POOL_SIZE: u8 = 192;
pub(super) const VECTOR_BASE: u8 = 0x32;

pub struct InterruptVectorAllocator{
    pub used: [Option<fn()>; VECTOR_POOL_SIZE as usize],
    starts_from: u8
}

impl InterruptVectorAllocator {
    pub const fn default() -> Self{
        InterruptVectorAllocator { 
            used: [None; VECTOR_POOL_SIZE as usize],
            starts_from: VECTOR_BASE
        }
    }

    pub fn set_and_get_free_vector(&mut self, interruptFn: fn()) -> Option<u8> {
        
        if IDT.get().is_none() {
            return None;
        }

        for i in 0..VECTOR_POOL_SIZE as usize{
            
            if self.used[i].is_none() { 
                
                self.used[i] = Some(interruptFn);

                return Some(i as u8 + VECTOR_BASE)
            }

        }

        None       
    }
}

