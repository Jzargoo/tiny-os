use core::sync::atomic::Ordering;


const VECTOR_POOL_SIZE: usize = 32;
const VECTOR_BASE: usize = 0x40;

pub struct InterruptVectorAllocator{
    pub used: [bool; VECTOR_POOL_SIZE],
    starts_from: usize
}

impl InterruptVectorAllocator {
    pub const fn default() -> Self{
        InterruptVectorAllocator { 
            used: [false; VECTOR_POOL_SIZE],
            starts_from: VECTOR_BASE
        }
    }

    pub fn get_free_interrupt_number(&mut self) -> usize {
        
        for i in 0..VECTOR_POOL_SIZE {
            
            if !self.used[i] {
                return i;
            }
        }

        0
    }
}