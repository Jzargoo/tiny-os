#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          // 4 kib
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
    
}

#[cfg(target_arch = "riscv64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}