use spin::Once;
use x86_64::{VirtAddr, structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode}};

use crate::{arch::x86_64::interrupts::interrupt_allocator::{VECTOR_BASE, VECTOR_POOL_SIZE}, println};


pub(super) static IDT: Once<InterruptDescriptorTable> = Once::new();



unsafe extern "C" {
    fn isr32(); fn isr33(); fn isr34(); fn isr35(); fn isr36(); fn isr37();
    fn isr38(); fn isr39(); fn isr40(); fn isr41(); fn isr42(); fn isr43();
    fn isr44(); fn isr45(); fn isr46(); fn isr47(); fn isr48(); fn isr49();
    fn isr50(); fn isr51(); fn isr52(); fn isr53(); fn isr54(); fn isr55();
    fn isr56(); fn isr57(); fn isr58(); fn isr59(); fn isr60(); fn isr61();
    fn isr62(); fn isr63(); fn isr64(); fn isr65(); fn isr66(); fn isr67();
    fn isr68(); fn isr69(); fn isr70(); fn isr71(); fn isr72(); fn isr73();
    fn isr74(); fn isr75(); fn isr76(); fn isr77(); fn isr78(); fn isr79();
    fn isr80(); fn isr81(); fn isr82(); fn isr83(); fn isr84(); fn isr85();
    fn isr86(); fn isr87(); fn isr88(); fn isr89(); fn isr90(); fn isr91();
    fn isr92(); fn isr93(); fn isr94(); fn isr95(); fn isr96(); fn isr97();
    fn isr98(); fn isr99(); fn isr100(); fn isr101(); fn isr102(); fn isr103();
    fn isr104(); fn isr105(); fn isr106(); fn isr107(); fn isr108(); fn isr109();
    fn isr110(); fn isr111(); fn isr112(); fn isr113(); fn isr114(); fn isr115();
    fn isr116(); fn isr117(); fn isr118(); fn isr119(); fn isr120(); fn isr121();
    fn isr122(); fn isr123(); fn isr124(); fn isr125(); fn isr126(); fn isr127();
    fn isr128(); fn isr129(); fn isr130(); fn isr131(); fn isr132(); fn isr133();
    fn isr134(); fn isr135(); fn isr136(); fn isr137(); fn isr138(); fn isr139();
    fn isr140(); fn isr141(); fn isr142(); fn isr143(); fn isr144(); fn isr145();
    fn isr146(); fn isr147(); fn isr148(); fn isr149(); fn isr150(); fn isr151();
    fn isr152(); fn isr153(); fn isr154(); fn isr155(); fn isr156(); fn isr157();
    fn isr158(); fn isr159(); fn isr160(); fn isr161(); fn isr162(); fn isr163();
    fn isr164(); fn isr165(); fn isr166(); fn isr167(); fn isr168(); fn isr169();
    fn isr170(); fn isr171(); fn isr172(); fn isr173(); fn isr174(); fn isr175();
    fn isr176(); fn isr177(); fn isr178(); fn isr179(); fn isr180(); fn isr181();
    fn isr182(); fn isr183(); fn isr184(); fn isr185(); fn isr186(); fn isr187();
    fn isr188(); fn isr189(); fn isr190(); fn isr191(); fn isr192(); fn isr193();
    fn isr194(); fn isr195(); fn isr196(); fn isr197(); fn isr198(); fn isr199();
    fn isr200(); fn isr201(); fn isr202(); fn isr203(); fn isr204(); fn isr205();
    fn isr206(); fn isr207(); fn isr208(); fn isr209(); fn isr210(); fn isr211();
    fn isr212(); fn isr213(); fn isr214(); fn isr215(); fn isr216(); fn isr217();
    fn isr218(); fn isr219(); fn isr220(); fn isr221(); fn isr222(); fn isr223();
    fn isr224();
}

macro_rules! collect_isrs {
    ($($isr:ident),* $(,)?) => {
        [
            $($isr as unsafe extern "C" fn(),)*
        ]
    };
}

#[doc(hidden)]
#[macro_export]
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

        idt.page_fault
            .set_handler_fn(page_fault_fn);

        idt.segment_not_present
            .set_handler_fn(segment_not_present_fn);

        
        let isrs = collect_isrs!(
            
            isr32, isr33, isr34, isr35, isr36, isr37, isr38, isr39, isr40, isr41, isr42, isr43, isr44, isr45, isr46, isr47,
            isr48, isr49, isr50, isr51, isr52, isr53, isr54, isr55, isr56, isr57, isr58, isr59, isr60, isr61, isr62, isr63, 
            isr64, isr65, isr66, isr67, isr68, isr69, isr70, isr71, isr72, isr73, isr74, isr75, isr76, isr77, isr78, isr79, 
            isr80, isr81, isr82, isr83, isr84, isr85, isr86, isr87, isr88, isr89, isr90, isr91, isr92, isr93, isr94, isr95, 
            isr96, isr97, isr98, isr99, isr100, isr101, isr102, isr103, isr104, isr105, isr106, isr107, isr108, isr109, isr110,
            isr111, isr112, isr113, isr114, isr115, isr116, isr117, isr118, isr119, isr120, isr121, isr122, isr123, isr124, 
            isr125, isr126, isr127, isr128, isr129, isr130, isr131, isr132, isr133, isr134, isr135, isr136, isr137, isr138, 
            isr139, isr140, isr141, isr142, isr143, isr144, isr145, isr146, isr147, isr148, isr149, isr150, isr151, isr152,
            isr153, isr154, isr155, isr156, isr157, isr158, isr159, isr160, isr161, isr162, isr163, isr164, isr165, isr166, 
            isr167, isr168, isr169, isr170, isr171, isr172, isr173, isr174, isr175, isr176, isr177, isr178, isr179, isr180, 
            isr181, isr182, isr183, isr184, isr185, isr186, isr187, isr188, isr189, isr190, isr191, isr192, isr193, isr194, 
            isr195, isr196, isr197, isr198, isr199, isr200, isr201, isr202, isr203, isr204, isr205, isr206, isr207, isr208, 
            isr209, isr210, isr211, isr212, isr213, isr214, isr215, isr216, isr217, isr218, isr219, isr220, isr221, isr222, 
            isr223, isr224

        );

        for (i, &isr_fn) in isrs.iter().enumerate() {
            
            let i = VECTOR_BASE + i as u8;

            let isr_fn_ptr = isr_fn as *const ();
            
            unsafe { 
                idt[i as u8].set_handler_addr(
                    VirtAddr::from_ptr(isr_fn_ptr)
                ).set_privilege_level(
                    x86_64::PrivilegeLevel::Ring0
                );
            }

        }

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


extern "x86-interrupt" fn page_fault_fn(frame: InterruptStackFrame, error_code: PageFaultErrorCode){
    threw_exception_code!(frame,error_code, "PAGE FAULT INTERRUPTER", "page fault. This is usually caused because of access to invalid page");
}