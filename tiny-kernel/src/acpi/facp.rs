use core::marker::PhantomData;

use crate::hal::addresses::PhysicalAddress;

pub struct Facp<P: PhysicalAddress>{
    _phantom: PhantomData<P>
}