use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Facp<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}

impl <P:PhysicalAddress> Facp<P> {
    pub fn new() -> Self{
        Facp { 
            _phantom: PhantomData 
        }
    }
}