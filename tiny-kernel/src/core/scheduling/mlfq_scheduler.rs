use crate::{arch::registers::ProcessRegisters, core::scheduling::{Scheduler, context_switch::context_switch, dl_list::Queue, process::Process}};

// MLFQ with feedback implementation

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
    current: *mut Process,
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
        let info = unsafe { (*self.current).scheduler_specific};

        if info.actual_monitor == 1 {
            
            if info.min_priority == info.actual_monitor{
                
                unsafe {
                    (*self.current).scheduler_specific.actual_monitor = 
                            self.levels[info.level_index].monitor_count
                }    

            } else {
                
                unsafe {

                    self.levels[info.level_index - 1].push_front(self.current);
                    
                    (*self.current).scheduler_specific.level_index -= 1;
                    
                    (*self.current).scheduler_specific.actual_monitor = self.levels[info.level_index - 1].monitor_count;
                }

            }

        } else {

            unsafe {
                
                (*self.current).scheduler_specific.actual_monitor -= 1
            
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

}

impl Scheduler for MlfqScheduler {
    
    unsafe fn schedule(&mut self) {

        let len = self.levels.len();
        
        unsafe {
            self.decrease_current_monitor()
        };

        let mut opt_process = None;

        let mut i= 0;

        while opt_process.is_none() && i < len {
            
            unsafe {
                opt_process = self.levels[i].pop_back()
            };

            i += 1; // before access to current level we should decrease it on 1
        
        }
        
        if let Some(process) = opt_process {
            
            let past = self.current;

            self.current = process as *mut Process;

            if self.is_time_to_boost(){
                self.boost();
            }            

            unsafe {
                context_switch(
                    &mut (*past).saved_registers as *mut ProcessRegisters, 
                    &(*self.current).saved_registers as *const ProcessRegisters
                );
            }   
        }  
    }
}