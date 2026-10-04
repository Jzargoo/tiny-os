pub mod process;
pub mod mlfq_scheduler;
pub mod context_switch;
pub mod dl_list;

pub trait Scheduler{
    unsafe fn schedule(&mut self);
}