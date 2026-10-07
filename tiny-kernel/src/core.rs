use alloc::boxed::Box;

use crate::{core::process::{process_control_block::PCB, process_structs::Process}, println};

pub mod process;

pub fn main() {
    let process = Process::from_current(2);

    let pid_maybe = PCB.lock().create_process(
        Box::new(process)
    );

    if let Some(pid) = pid_maybe {
    
        unsafe {
            println!("Setting ready");
            PCB.lock().set_ready(pid);
        }
    
        
        for i in 0..30000{
            
            if i == 10000 {
                println!("{}", i);   
            }

        }

        
        if let Some(proc) = PCB.lock().get_process(pid) {
            println!("{:?}", proc)
        }

        panic!("TEST PANIC");
    
    }
}