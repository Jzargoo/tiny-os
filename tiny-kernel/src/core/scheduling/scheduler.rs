use alloc::vec::Vec;

use crate::core::scheduling::process::Process;

pub struct Scheduler {
    levels: [Queue; 12],
    monitor: [u32; 12],
    current_pid: usize
}

pub struct Queue{
    internal: Vec<Process>,
    counter: u64
}

impl Queue {
    
    pub fn new() -> Self{
        Self {
            internal: Vec::new(),
            counter: 0
        }
    }

    pub fn get_next(&mut self) -> Option<&Process>{
        self.counter += 1;

        let len = self.internal.len();

        self.internal.get(len % self.counter as usize)
    }

}

impl Scheduler {

    pub fn schedule(&mut self) {
        let len = self.levels.len();
        
        let mut opt_process = None;

        let mut i= 0;

        while opt_process.is_none() && i < len {
            
            opt_process = self.levels[i].get_next();
            

            i += 1; // before access to current level we should decrease it on 1
        
        }

        if let Some(process) = opt_process {

        }   


    }

    pub fn boost(&mut self) {

        let mut tmp_buffer: Vec<Process> = Vec::new();
        
        for queue in &mut self.levels {
            tmp_buffer.append(&mut queue.internal);    
        }

        self.levels[0].internal = tmp_buffer;
        
    }

}