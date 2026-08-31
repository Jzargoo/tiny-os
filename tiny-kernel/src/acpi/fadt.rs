use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Fadt<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}

impl <P:PhysicalAddress> Fadt<P> {
    pub fn new() -> Self{
        Self { 
            _phantom: PhantomData 
        }
    }
}