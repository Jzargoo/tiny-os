use core::arch::naked_asm;

use crate::arch::registers::ProcessRegisters;

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