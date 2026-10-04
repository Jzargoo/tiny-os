use crate::arch::registers::ProcessRegisters;

#[allow(dead_code)]
pub struct Process {
    pid: usize,
    state: u8,
    pub saved_registers: ProcessRegisters,
    pub scheduler_specific: ProcessSchedulingInfo
}

#[derive(Clone, Copy)]
pub struct ProcessSchedulingInfo{
    pub actual_monitor: usize,
    pub level_index: usize,
    pub min_priority: usize,
    pub next: *mut Process,
    pub previous: *mut Process,
}