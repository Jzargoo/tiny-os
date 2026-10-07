use crate::{core::process::process_structs::ProcessSchedulingInfo, force_println};
use core::ptr::null_mut;

#[derive(Debug)]
pub struct Queue {
    pub head: *mut ProcessSchedulingInfo,
    pub tail: *mut ProcessSchedulingInfo,
    pub monitor_count: usize,
}

impl Queue {
    pub const fn new(monitor: usize) -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
            monitor_count: monitor,
        }
    }
    pub const fn default() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
            monitor_count: 1
        }
    }

    // Push front:
    //
    // head -> A -> B -> C <- tail
    //
    // push_front(D):
    //
    // head -> D -> A -> B -> C <- tail
    pub unsafe fn push_front(&mut self, process: *mut ProcessSchedulingInfo) {
        debug_assert!(!process.is_null());

        unsafe {   
            (*process).previous = null_mut();
            (*process).next = self.head;
        };

        if self.head.is_null() {
        
            self.tail = process;
        
        } else {
            
            unsafe {
                (*self.head).previous = process
            };

        }

        self.head = process;
    }

    // Take from the end:
    //
    // head -> A -> B -> C <- tail
    //
    // pop_back() -> C
    //
    // head -> A -> B <- tail
    pub unsafe fn pop_back(&mut self) -> Option<*mut ProcessSchedulingInfo> {
        force_println!(
            "POP_BACK BEFORE: head={:?}, tail={:?}",
            self.head,
            self.tail
        );

        if self.tail.is_null() {
            
            force_println!("Queue is empty!");

            return None;
        }

        let process = self.tail;
        
        let new_tail = unsafe {
            (*process).previous
        };

        if new_tail.is_null() {
        
            self.head = null_mut();
            self.tail = null_mut();
        
        } else {
            unsafe { 
                (*new_tail).next = null_mut(); 
            }

            self.tail = new_tail;
        }

        unsafe{
            (*process).previous = null_mut();
            (*process).next = null_mut();
        }

        Some(process)
    }

    /// Appends all processes from `other` to the end of `self`.
    ///
    /// self:  A <-> B
    /// other: C <-> D
    ///
    /// result: A <-> B <-> C <-> D
    ///
    /// `other` becomes empty.
    pub unsafe fn append(&mut self, other: &mut Queue) {
        
        if other.head.is_null() {
            return;
        }

        if self.tail.is_null() {
        
            // self is empty
        
            self.head = other.head;
            self.tail = other.tail;
        
        } else {

            // Merge
            
            unsafe {
                (*self.tail).next = other.head;
                (*other.head).previous = self.tail;
            }

            self.tail = other.tail;
        }

        // other is empty
        other.head = null_mut();
        other.tail = null_mut();
    
    }
}