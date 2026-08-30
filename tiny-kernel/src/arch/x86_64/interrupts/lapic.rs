use lapic::{LocalApic, TimerLVT};
use spin::Mutex;
use x86_64::VirtAddr;

use crate::arch::{scheduling, x86_64::interrupts::VECTOR_INTERRUPT_ALLOCATOR};
pub struct ApicDriver {
    apic: &'static mut LocalApic
}

pub static APIC_DRIVER: Mutex<Option<ApicDriver>>  = Mutex::new(None);

pub unsafe fn raw_send_eoi() {
    let eoi_ptr = (0xFEE0_0000 as usize + 0xB0) as *mut u32;
    
    unsafe { eoi_ptr.write_volatile(0) };
}

impl ApicDriver {

    pub fn enable(&self){
        let mut iv = self.apic.spurious_iv;

        iv.set_apic_enabled(0x1);
        
        iv.set_spurious_vector(0xFF);
    }

    pub unsafe fn new(addr: VirtAddr) -> Self{
        let apic = unsafe { &mut *(addr.as_mut_ptr() as *mut LocalApic) };
        Self { apic }
    }

    pub unsafe fn setup_timer(&self) -> Option<u8> {

        let lvn = VECTOR_INTERRUPT_ALLOCATOR.lock().set_and_get_free_vector(
            scheduling::schedule
        );

        if let Some(lvt) = lvn {

            self.apic.timer_lvt = TimerLVT::new();
        }

        lvn
    }
}

