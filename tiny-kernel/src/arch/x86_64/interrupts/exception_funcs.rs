use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

static IDT: Once<InterruptDescriptorTable> = Once::new();


macro_rules! threw_exception {
    ($frame:expr) => {
        $crate::logger
        println!("[Generic Interrupter] Fatal error at {:?}", $frame)
        
        panic!("PANIC FROM GENERIC EXCEPTION INTERRUPTER")
    };

    ($frame:expr, $interrupter:expr, $($arg:tt)*) => {{
        println!("[{}] {} at {:?}", $interrupter, format_args!($($arg)*), $frame);
        panic!("PANIC FROM {}", $interrupter)
    }};
}

macro_rules! threw_exception_code {
    ($frame:expr, $error_code:expr, $interrupter:expr, $($arg:tt)*) => {{
        println!(
            "[{}] {} (Error Code: {:#x}) at {:?}", 
            $interrupter, 
            format_args!($($arg)*), 
            $error_code,
            $frame
        );
        panic!("PANIC FROM {}", $interrupter);
    }};
}


pub(in crate::arch::x86_64::interrupts) fn setup_idt() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
    
        idt.breakpoint.set_handler_fn(breakpoint_fn);

        idt.divide_error
            .set_handler_fn(division_error_fn);
        
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_fn);

        idt.overflow
            .set_handler_fn(overflow_fn);
        
        idt.invalid_opcode
            .set_handler_fn(invalid_opcode_fn);

        idt.invalid_tss
            .set_handler_fn(invalid_tss_fn);
        
        unsafe {
            
            idt.double_fault
            .set_handler_fn(double_fault_fn)
            .set_stack_index(1);
        
        }

        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_fn);

        idt.device_not_available
            .set_handler_fn(device_not_available_fn);

        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_fn);

        idt.device_not_available
            .set_handler_fn(device_not_available_fn);

        idt.segment_not_present
            .set_handler_fn(segment_not_present_fn);

        idt
    });

    idt.load();
}



extern "x86-interrupt" fn breakpoint_fn(frame: InterruptStackFrame) {
    println!("[BREAKPOINT INTERRUPTER] interrupt frame is {:?}. This function was invoked by a CPU not directly!", frame)
}

extern "x86-interrupt" fn division_error_fn(frame: InterruptStackFrame) {
    threw_exception!(frame, "DIVISION ERROR INTERRUPTER", "there was a division error(probably division by zero)");
}

extern "x86-interrupt" fn overflow_fn(frame: InterruptStackFrame) {
    threw_exception!(frame, "OVERFLOW INTERRUPTER", "the result of the arithmetic operation caused it to overflow. The result is larger than the value can store");
}

extern "x86-interrupt" fn bound_range_exceeded_fn(frame: InterruptStackFrame) {
    threw_exception!(frame, "BOUND RANGE EXCEEDED INTERRUPTER", "the bound range was exceeded. This is usually caused by an array index being out of bounds");
}

extern "x86-interrupt" fn invalid_opcode_fn(frame: InterruptStackFrame) {
    threw_exception!(frame, "INVALID OPCODE INTERRUPTER", "CPU encountered an invalid instruction. This is usually caused by executing data as code or executing an instruction that is not supported by the CPU");
}

extern "x86-interrupt" fn device_not_available_fn(frame: InterruptStackFrame) {
    threw_exception!(frame, "DEVICE NOT AVAILABLE INTERRUPTER", "the device is not available. This is usually caused by the FPU being disabled");
}

extern "x86-interrupt" fn invalid_tss_fn(frame: InterruptStackFrame, error_code: u64) {
    threw_exception_code!(frame, error_code, "INVALID TSS INTERRUPTER", "the TSS is invalid. This is usually caused by a task switch to a task that has an invalid TSS");
}

extern "x86-interrupt" fn segment_not_present_fn(frame: InterruptStackFrame, error_code: u64) {
    threw_exception_code!(frame,error_code, "SEGMENT NOT PRESENT INTERRUPTER", "the segment is not present. This is usually caused by accessing a segment that has not been loaded");
}

extern "x86-interrupt" fn stack_segment_fault_fn(frame: InterruptStackFrame, error_code: u64) {
    threw_exception_code!(frame,error_code, "STACK SEGMENT FAULT INTERRUPTER", "the stack segment fault occurred. This is usually caused by a stack overflow or an invalid stack segment");
}

extern "x86-interrupt" fn general_protection_fault_fn(frame: InterruptStackFrame, error_code: u64) {
    threw_exception_code!(frame, error_code, "GENERAL PROTECTION FAULT INTERRUPTER", "a general protection fault occurred. This is usually caused by an invalid memory access or an attempt to execute an instruction that is not allowed in the current privilege level");
}

extern "x86-interrupt" fn double_fault_fn(frame: InterruptStackFrame, error_code: u64) -> ! {
    threw_exception_code!(frame, error_code, "DOUBLE FAULT INTERRUPTER", "a double fault occurred. This is usually caused when a previous exception handler fails to handle an exception and the CPU cannot recover from it");
}
