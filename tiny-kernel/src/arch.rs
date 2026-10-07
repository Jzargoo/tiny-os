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
    use core::arch::naked_asm;

    
    #[repr(C, packed)]
    #[derive(Clone, Copy)]
    #[derive(Debug)]
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

    impl ProcessRegisters {
        pub unsafe fn  from_current() -> Self{
            let mut registers = Self { rbx: 0, rbp: 0, r12: 0, r13: 0, r14: 0, r15: 0, rsp: 0, rip: 0, cr3: 0 };
            
            unsafe {
                save_context(&mut registers as *mut ProcessRegisters);
            }
            
            registers
        } 
    }

    #[unsafe(naked)]
    pub unsafe extern "C" fn context_switch(
        old_registers: *mut ProcessRegisters,
        new_registers: *const ProcessRegisters 
    ) {
        naked_asm!(
        
            "mov [rdi + 0], rbx",
            "mov [rdi + 8], rbp",
            "mov [rdi + 16], r12",
            "mov [rdi + 24], r13",
            "mov [rdi + 32], r14",
            "mov [rdi + 40], r15",
        
            "mov rax, [rsp]", 
            "mov [rdi + 56], rax",

            "lea rax, [rsp + 8]",
            "mov [rdi + 48], rax",
        
            "mov rax, cr3",
            "mov [rdi + 64], rax",

            "mov rax, [rsi + 64]",
            "mov cr3, rax",       

            "mov rbx, [rsi + 0]",
            "mov rbp, [rsi + 8]",
            "mov r12, [rsi + 16]",
            "mov r13, [rsi + 24]",
            "mov r14, [rsi + 32]",
            "mov r15, [rsi + 40]",
        
            "mov rsp, [rsi + 48]",
        
            "mov rax, [rsi + 56]",
            "push rax",
        
            "ret",
        );
    }
 
    // previous is null

    #[unsafe(naked)]
    pub unsafe extern "C" fn change_context(
        new_registers: *const ProcessRegisters,
    ) {
        naked_asm!(
            // Load address space
            "mov rax, [rdi + 64]",
            "mov cr3, rax",

            // Load callee-saved registers
            "mov rbx, [rdi + 0]",
            "mov rbp, [rdi + 8]",
            "mov r12, [rdi + 16]",
            "mov r13, [rdi + 24]",
            "mov r14, [rdi + 32]",
            "mov r15, [rdi + 40]",

            // Load stack
            "mov rsp, [rdi + 48]",

            // Return address
            "mov rax, [rdi + 56]",
            "push rax",

            // Jump to new process
            "ret",
        );
    }

    #[unsafe(naked)]
    pub unsafe extern "C" fn save_context(
        registers: *mut ProcessRegisters
    ) {
        naked_asm!(
            // Save callee-saved registers
            "mov [rdi + 0], rbx",
            "mov [rdi + 8], rbp",
            "mov [rdi + 16], r12",
            "mov [rdi + 24], r13",
            "mov [rdi + 32], r14",
            "mov [rdi + 40], r15",

            // Save RIP from return address
            "mov rax, [rsp]",
            "mov [rdi + 56], rax",

            // Save RSP after returning from save_context
            "lea rax, [rsp + 8]",
            "mov [rdi + 48], rax",

            // Save address space
            "mov rax, cr3",
            "mov [rdi + 64], rax",

            "ret",
        )
    }

}

#[cfg(target_arch = "riscv64")]
pub mod pages {
    pub const PAGE_SIZE_REGULAR: usize = 4096;          
    pub const PAGE_SIZE_LARGE: usize = 1024 * 1024 * 2; // 2 mib
    pub const PAGE_SIZE_HUGE: usize = 1024 * 1024 * 1024;  // 1 gib
}