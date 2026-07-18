use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Hpet<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}