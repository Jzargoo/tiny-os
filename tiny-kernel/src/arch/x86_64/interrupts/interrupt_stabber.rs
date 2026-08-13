use core::arch::naked_asm;

use crate::{arch::x86_64::interrupts::{VECTOR_INTERRUPT_ALLOCATOR, lapic::lapic_send_eoi}, println, threw_exception};


#[unsafe(naked)]
pub unsafe extern "C" fn isr_common_stub() {
    naked_asm!(
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",

        "mov rdi, rsp",

        "call {handler}",

        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",

        "add rsp, 16",

        "iretq",

        handler = sym common_interrupt_fn
    )
}


pub fn common_interrupt_fn (frame: InterruptStackFrameWithVectorNumber) {
    
    let number = frame.vector_number;
    let index = number.saturating_sub(32);
    
    let isr_fn_opt = VECTOR_INTERRUPT_ALLOCATOR
        .lock()
        .used[index as usize];

    if let Some(isr_fn) = isr_fn_opt {

        isr_fn();
        lapic_send_eoi();


    } else {

        lapic_send_eoi();
        
        threw_exception!(
            frame,
            "INTERRUPT STABBER", 
            "incorrect vector number was invoked with. Isr was registered but subsequent function is missing in vector allocator"
        );

    }
}

#[repr(C)]
#[derive(Debug)]
struct InterruptStackFrameWithVectorNumber {

    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    pub error_code: u64,
    pub vector_number: u64,

    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}