use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Mcfg<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}