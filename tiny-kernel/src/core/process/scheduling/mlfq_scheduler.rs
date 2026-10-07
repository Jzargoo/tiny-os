use core::ptr::null_mut;

use crate::{arch::registers::{ProcessRegisters, change_context, context_switch}, core::process::{process_structs::ProcessSchedulingInfo, scheduling::{Scheduler, dl_list::Queue}}, force_println, println};

// MLFQ implementation

// Priority level determines which queue gets processor time first

// The maximum continuous CPU time a process can use in this queue before being preempted

// Scheduling policy typically Round Robin with a FIFO ready line within the same priority level

// Monitor max number of times a process can be scheduled (chosen from this queue) before automatic demotion
// remaining_monitor decremented by 1 every time the process is chosen by the scheduler.
// when it reaches 1, the process priority is decremented (demoted to lower queue if its current priority > minPriority)

// Priority Boost interval a periodic reset mechanism that moves all processes back 
// to the highest-priority queue to prevent starvation of CPU-bound tasks   

pub struct MlfqScheduler {
    levels: [Queue; 12],
    current: *mut ProcessSchedulingInfo,
    count_to_boost: usize
}

impl MlfqScheduler {

    pub fn boost(&mut self) {

        let mut tmp_buffer= Queue::new(
            self.levels[0].monitor_count
        );
        
        for queue in &mut self.levels {
        
            unsafe {
                tmp_buffer.append(queue);
            }      
        
        }

        self.levels[0] = tmp_buffer;

    }

    unsafe fn decrease_current_monitor(&mut self){
        let info = unsafe { *self.current };

        if info.actual_monitor == 1 {
            
            if info.min_priority == info.actual_monitor{
                
                unsafe {
                    (*self.current).actual_monitor = 
                            self.levels[info.level_index].monitor_count
                }    

            } else {
                
                unsafe {

                    self.levels[info.level_index - 1].push_front(self.current);
                    
                    (*self.current).level_index -= 1;
                    
                    (*self.current).actual_monitor = self.levels[info.level_index - 1].monitor_count;
                }

            }

        } else {

            unsafe {
                
                (*self.current).actual_monitor -= 1
            
            }

        }
    }

    fn is_time_to_boost(&mut self) -> bool{
        
        if self.count_to_boost <= 0 {
        
            self.count_to_boost = 100;
        
            true
        } else {
            self.count_to_boost -= 1;
        
            false
        }

    }

    pub const fn new() -> Self {
        let levels = [
            Queue::new(1), Queue::new(6), Queue::new(9),
            Queue::new(12), Queue::new(15), Queue::new(18),
            Queue::new(21), Queue::new(24), Queue::new(27),
            Queue::new(30), Queue::new(33), Queue::new(36)
        ];


        MlfqScheduler { 
            levels: levels, current: null_mut(), count_to_boost: 100
        }

    }

    pub unsafe fn add_process(
        &mut self,
        process: *mut ProcessSchedulingInfo
    ){

        #[cfg(debug_assertions)]
        println!("adding a new process into scheduler");
        
        unsafe{
            (*process).actual_monitor = self.levels[0].monitor_count;
            (*process).level_index = 0;
        }

        unsafe {
            self.levels[0]
                .push_front(process)
        }

        println!(
            "AFTER PUSH: head = {:p}, tail = {:p}",
            self.levels[0].head,
            self.levels[0].tail
        );
    }
}

impl Scheduler for MlfqScheduler {
    
    unsafe fn schedule(&mut self) {
        
        force_println!(
            "SCHEDULE: L0 head={:?}, tail={:?}",
            self.levels[0].head,
            self.levels[0].tail
        );
        
        let len = self.levels.len();

        let mut opt_process = None;

        let mut i= 0;

        while opt_process.is_none() && i < len {
            
            unsafe {
                opt_process = self.levels[i].pop_back()
            };

            i += 1; // before access to current level we should decrease it on 1
        
        }

        if let Some(process) = opt_process {

            #[cfg(debug_assertions)]
            force_println!("WE FOUND a new process!!");

            let past = self.current;

            self.current = process as *mut ProcessSchedulingInfo;

            if self.is_time_to_boost(){
                #[cfg(debug_assertions)]
                force_println!("ITS BOOSTING TIME!!");
                //self.boost();
            }            

            if past.is_null() {
            
                unsafe {
                    #[cfg(debug_assertions)]
                    force_println!("SWITCHING CONTEXT!!");
                    change_context(
                        &(*self.current).saved_registers as *const ProcessRegisters
                    );
                }
            
            } else {
                
                unsafe {
                    #[cfg(debug_assertions)]
                    force_println!("SWITCHING CONTEXT!!");
                    context_switch(
                        &mut (*past).saved_registers as *mut ProcessRegisters, 
                        &(*self.current).saved_registers as *const ProcessRegisters
                    );
                }

            }
        }
    }  
}