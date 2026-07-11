#[cfg(target_arch = "x86_64")]
pub mod x86_64;



#[cfg(target_arch = "x86_64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          // 4 kib
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}



#[cfg(target_arch = "x86_64")]
pub mod devices {
    use crate::hal::pci_device::PciDevice;
    use crate::arch::x86_64::interrupts::interrupts_funcs;

    pub fn is_device_exist(bus: u8, slot: u8, func: u8) -> bool{
        interrupts_funcs::is_device_exist(bus, slot, func)
    }

    pub fn init_configuration(bus: u8, slot: u8, func: u8) -> PciDevice {
        interrupts_funcs::init_configuration(bus, slot, func)
    }

    pub fn is_milti_func(bus:u8, slot: u8) -> bool{
        interrupts_funcs::is_multi_func(bus, slot)
    }
}


#[cfg(target_arch = "riscv64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}