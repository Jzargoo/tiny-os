pub trait PhysicalAddress: Copy + Clone {
    fn to_u64(&self) -> u64;
    
    fn is_valid(&self) -> bool;

    fn from_u64(addr: u64) -> Self; 
    
    type VAddr: VirtualAddress;
    
    fn to_virtual(&self, offset: u64) -> Option<Self::VAddr>;
}

pub trait VirtualAddress: Copy + Clone {
    fn to_u64(&self) -> u64;
    fn is_higher_half(&self) -> bool;
    
    unsafe fn to_ptr<T>(&self) -> *const T;
}