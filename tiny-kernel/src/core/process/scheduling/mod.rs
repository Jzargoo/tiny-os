use spin::Mutex;

use crate::core::process::scheduling::mlfq_scheduler::MlfqScheduler;

pub mod mlfq_scheduler;
pub mod dl_list;

pub trait Scheduler{
    unsafe fn schedule(&mut self);
}

pub const SCHEDULER: Mutex<MlfqScheduler> = Mutex::new(
    MlfqScheduler::new()
);