use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Madt<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}