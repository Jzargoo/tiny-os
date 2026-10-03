#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod header_specific_pci;

pub mod pci_device;

pub mod device_capibilities;

pub mod enhanced_pci_mechanism;

pub mod scheduling;


#[cfg(target_arch = "x86_64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          // 4 kib
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}

#[cfg(target_arch = "x86_64")]
pub mod registers {
    
    #[repr(C, packed)]
    pub struct ProcessRegisters {
        pub rbx: u64,
        pub rbp: u64,
        pub r12: u64,
        pub r13: u64,
        pub r14: u64,
        pub r15: u64,
    
        pub rsp: u64,
        pub rip: u64,
        pub cr3: u64,
    }

}

#[cfg(target_arch = "riscv64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}