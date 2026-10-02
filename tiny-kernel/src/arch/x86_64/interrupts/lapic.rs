use core::{ops::Add, ptr::{read_volatile, write_volatile}};

use spin::Once;
use x86_64::VirtAddr;

use crate::{arch::x86_64::interrupts::{interrupt_stabber::SPURIOUS_VECTOR_NUMBER, lapic_requests_options::TimerOptions}};

const APIC_EOI: usize         = 0x0B0;
const APIC_SVR: usize         = 0x0F0;
const APIC_LVT_TIMER: usize   = 0x320;
const APIC_INITIAL_COUNT: usize = 0x380;
const APIC_CURRENT_COUNT: usize = 0x390;
const APIC_DIVIDE_CONFIG: usize = 0x3E0;

pub struct ApicDriver {
    apic: VirtAddr
}

pub static APIC_DRIVER: Once<ApicDriver> = Once::new();

// General methods
impl ApicDriver{
    
    pub unsafe fn read(&self, offset: u64) -> u32 {
        
        unsafe {
            read_volatile(
                self.apic.add(offset).as_mut_ptr()
            )
        }

    }

    pub unsafe fn write(&self, offset: u64, value: u32) {

        unsafe {
            write_volatile(
                self.apic.add(offset).as_mut_ptr(),
                value
            );
        }

    }

    pub fn new(base: VirtAddr) -> Self{
        Self { apic: base }
    }

}

// Setting up spiroius vector register
impl ApicDriver {

    pub unsafe fn setup_spurious_vector(&self) {

        // Setup vector interrupt number for SPR
        unsafe { 
            // SPR register
            let value: u32 = (SPURIOUS_VECTOR_NUMBER as u32) | (1 << 8);

            self.write(APIC_SVR as u64, value);
        }

    }

}

// Reading and setting up a timer. 3 registers: lvt, count, divisor
impl ApicDriver {

    unsafe fn setup_timer_count(&self, count: u32){
        
        unsafe {
            self.write(APIC_INITIAL_COUNT as u64, count)
        };

    }

    unsafe fn setup_timer_lvt(&self, vin: u8, del_status: u8, mask: u8, timer_mode: u8){
        let timer_lvt: u32 = 
                vin as u32 | 
                    (del_status as u32  & 0b1) << 12 | 
                    (mask as u32  & 0b1) << 16 | 
                    (timer_mode as u32  & 0b11) << 17;

        unsafe {
            self.write(APIC_LVT_TIMER as u64, timer_lvt);
        }
    }

    unsafe fn setup_timer_div_config(&self, divisor: u8){

        let reg_value = (divisor as u32) & 0xF;

        unsafe {
            self.write(APIC_DIVIDE_CONFIG as u64, reg_value)
        }

    }

    pub unsafe fn setup_timer(&self, options: TimerOptions){
        
        unsafe {
            self.setup_timer_lvt(
                options.get_vector(),
                options.get_delivery_status(), 
                options.get_mask(), 
                options.get_timer_mode()
            );

            self.setup_timer_div_config(options.get_div());

            self.setup_timer_count(options.get_count());
        }

    }

    pub unsafe fn read_current_count(&self) -> u32{
    
        unsafe {
            self.read(APIC_CURRENT_COUNT as u64)
        }
    
    } 

}

// Sending End of interrupt
impl ApicDriver {
    
    pub unsafe fn send_eoi(&self){
        
        unsafe{
            self.write(APIC_EOI as u64, 0)
        }

    }

}