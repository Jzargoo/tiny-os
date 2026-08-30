use x86_64::{PhysAddr, VirtAddr};

use crate::hal::addresses::{PhysicalAddress, VirtualAddress};

pub mod page_allocator;

pub mod boot;

pub mod interrupts;

impl VirtualAddress for VirtAddr {
    fn to_u64(&self) -> u64 {
        self.as_u64()
    }

    fn is_higher_half(&self) -> bool {
        self.to_u64() >= 0xFFFF_8000_0000_0000
    }

    unsafe fn to_ptr<T>(&self) -> *const T {
        self.as_ptr()
    }
}

impl PhysicalAddress for x86_64::PhysAddr {
    fn to_u64(&self) -> u64 {
        self.as_u64()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn from_u64(addr: u64) -> Self {
        PhysAddr::new(addr)
    }

    type VAddr = VirtAddr;

    fn to_virtual(&self, offset: u64) -> Option<Self::VAddr> {
        Some(
            VirtAddr::new(
                self.as_u64() + offset 
            )
        )
    }
}

