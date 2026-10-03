use crate::arch::registers::ProcessRegisters;

#[allow(dead_code)]
pub struct Process {
    pid: usize,
    state: u8,
    saved_registers: ProcessRegisters,
    priority: u8
}