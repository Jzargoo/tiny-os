use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn setup_idt() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
    
        idt.breakpoint.set_handler_fn(breakpoint_fn);
        
        idt
    });

    idt.load();
}

extern "x86-interrupt" fn breakpoint_fn(frame: InterruptStackFrame) {
    println!("[BREAKPOINT INTERRUPTER] interrupt frame is {:?}. This function was invoked by a CPU not directly!", frame)
}