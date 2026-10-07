use core::ptr::null_mut;

use crate::{arch::registers::ProcessRegisters, core::process::process_structs::State::ZOMBIE};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Process {
    pub pid: usize,
    pub scheduler_specific: ProcessSchedulingInfo,
    pub state: State
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    INITED, READY, RUNNING, BLOCKED, DELETED, ZOMBIE, GHOST
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct ProcessSchedulingInfo{
    pub actual_monitor: usize,
    pub level_index: usize,
    pub min_priority: usize,
    pub saved_registers: ProcessRegisters,
    pub next: *mut ProcessSchedulingInfo,
    pub previous: *mut ProcessSchedulingInfo,
}

impl Process{
    pub fn from_current(min_priortity: usize) -> Self {
        
        let registers = unsafe {ProcessRegisters::from_current()};
        
        Self { 
            pid: 0, 
            scheduler_specific: ProcessSchedulingInfo { 
                actual_monitor: 0, 
                level_index: 0, 
                min_priority: min_priortity, 
                saved_registers: registers, 
                next: null_mut(), 
                previous: null_mut() 
            }, 
            state: ZOMBIE 
        }

    }

}

unsafe impl Send for ProcessSchedulingInfo {}