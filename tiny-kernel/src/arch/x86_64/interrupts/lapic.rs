
use core::ptr::{addr_of, read_volatile, write_volatile};

use lapic::{LocalApic, TimerLVT};

use spin::Once;
use x86_64::VirtAddr;

use crate::arch::x86_64::interrupts::lapic_requests_options::TimerOptions;

pub struct ApicDriver {
    apic: &'static mut LocalApic
}

pub static APIC_DRIVER: Once<ApicDriver> = Once::new();

impl ApicDriver {

    pub unsafe fn enable(&self){
        
        let ptr = addr_of!(self.apic.spurious_iv).cast::<u32>();
        
        unsafe {
            
            let val = read_volatile(ptr);

            let turned_on = val | (1u32 << 8);

            ptr.cast_mut().write_volatile(turned_on);

        };

    }

    pub unsafe fn new(addr: VirtAddr) -> Self{

        let apic = unsafe { 
            &mut *(addr.as_mut_ptr() as *mut LocalApic) 
        };
        
        Self { apic }
    }

    pub unsafe fn send_eoi(&self){
        
        unsafe {
            let eoi = 
                core::ptr::addr_of!(self.apic.eoi).cast::<u32>();

            write_volatile(eoi.cast_mut(), 0);

        }

    }

    pub unsafe fn setup_timer(&self, options: TimerOptions) {
    
        let mut timer = TimerLVT::new();


        timer.set_vector(options.get_vector());
            
        timer.set_timer_mode(options.get_timer_mode());

        timer.set_delivery_status(options.get_delivery_status());

        timer.set_mask(options.get_mask());


        unsafe { 
            
            addr_of!(self.apic.timer_lvt)
                .cast_mut()
                .write_volatile(timer);
        
        };

    }
}

