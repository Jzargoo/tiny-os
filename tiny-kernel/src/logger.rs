
use spin::Mutex;
use logging::Logger;

use crate::logger::serial::writer::FALLBACK;
pub mod graphycal;
pub mod serial;
pub mod ring_buffer;

pub mod logging;


pub static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());

#[macro_export]
macro_rules! panic_flush {
    () => {
        crate::logger::_panic_flush();
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::logger::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::logger::_print(format_args!($($arg)*));
        $crate::print!("\n");
    }};
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    
    let mut logger = LOGGER.lock(); 
    
    let _ = logger.write_fmt(args);
    
}

#[doc(hidden)]
pub fn _panic_flush(){
    
    FALLBACK.write_string("Panic flush: ");

    unsafe {
        LOGGER.force_unlock();
    };
    
    let mut log = LOGGER.lock();

    
    if log.is_sinks_inited() {
        log.flush_all();
    } else {

        let mut line = log.read();
        
        FALLBACK.write_string("Panic flush: ");

        while line.is_some() {
            FALLBACK.write_string(line.unwrap());
            line = log.read();
        }
    }
}

