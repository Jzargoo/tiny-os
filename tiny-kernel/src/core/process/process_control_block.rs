use alloc::{boxed::Box, vec::{self, Vec}};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{core::process::{self, process_structs::{Process, ProcessSchedulingInfo, State::{INITED, READY}}, scheduling::SCHEDULER}, println};

pub struct ProcessRegistry {
    processes: Vec<Option<Box<Process>>>
}

lazy_static!{
    pub static ref PCB: Mutex<ProcessRegistry> = Mutex::new(ProcessRegistry::init());
}


impl ProcessRegistry {

    pub fn init()-> Self {

        let mut vector = Vec::<Option<Box<Process>>>::new();

        vector.push(None);

        Self { 
            processes: vector   
        }

    }

    pub fn get_process(&self, pid: usize) -> Option<&Box<Process>> {

        if  let Some(process_maybe) = self.processes.get(pid) && 
            let Some(process) = process_maybe {
            
                Some(process)
            
            } else {
                None
            }
        
    }

    pub fn create_process(&mut self, mut process: Box<Process>) -> Option<usize>{
        let len = self.processes.len();

        for i in 0..len {
            
            if let Some(indexed_process) = self.processes.get(i){
                
                if indexed_process.is_none() {
                
                    process.pid = i;
                
                    process.state = INITED;
                
                    self.processes.insert(i, Some(process));
                
                    return Some(i);
                
                }
            }
        }
        None
    }
        
    pub unsafe fn set_ready(&mut self, pid: usize) {
        
        if  let Some(el) = self.processes.get_mut(pid) && 
            let Some(process) = el {
                
                println!("found a process with pid {}", pid);
                
                process.state = READY;

                unsafe{
                    SCHEDULER.lock().add_process(
                        &mut process.scheduler_specific as *mut ProcessSchedulingInfo
                    )
                }

        }

    }

}