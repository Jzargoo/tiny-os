use core::arch::{global_asm, naked_asm};


use crate::{arch::x86_64::interrupts::{VECTOR_INTERRUPT_ALLOCATOR, interrupt_allocator::VECTOR_BASE, lapic::lapic_send_eoi}, println, threw_exception};


macro_rules! make_isr_no_err {
    ($num:expr) => {
        global_asm!(
            concat!(
                ".global isr", stringify!($num), "\n",
                "isr", stringify!($num), ":\n",
                "    cli\n",
                "    push 0\n",
                "    push ", stringify!($num), "\n",
                "    jmp isr_common_stub\n"
            )
        );
    };
}

macro_rules! make_isr_range {
    ($start:expr, $end:expr) => {
        make_isr_no_err!($start);
        #[cfg(panic = "never")] 
        const _: () = ();
    };
}

macro_rules! generate_isrs {
    ($($val:expr),*) => {
        $( make_isr_no_err!($val); )*
    };
}

generate_isrs!(
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224
);
#[unsafe(no_mangle)]
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


unsafe extern "C" fn common_interrupt_fn (frame: *mut InterruptStackFrameWithVectorNumber) {
    
    let number = unsafe { (*frame).vector_number };
    
    let index = number.saturating_sub(VECTOR_BASE as u64);

    
    
    let isr_fn_opt = VECTOR_INTERRUPT_ALLOCATOR
        .lock()
        .used[index as usize];

    if let Some(isr_fn) = isr_fn_opt {

        isr_fn();

        unsafe { lapic_send_eoi() };

    } else {

        unsafe { lapic_send_eoi() };

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
