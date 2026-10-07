use crate::{core::process::scheduling::{SCHEDULER, Scheduler}};

pub fn schedule() {

    unsafe{
        if let Some(mut scheduler) = SCHEDULER.try_lock() {
            scheduler.schedule();
        }
    }
    
}