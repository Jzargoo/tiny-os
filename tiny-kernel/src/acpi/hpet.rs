use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Hpet<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}


impl <P:PhysicalAddress> Hpet<P> {
    pub fn new() -> Self{
        Hpet{ 
            _phantom: PhantomData 
        }
    }
}