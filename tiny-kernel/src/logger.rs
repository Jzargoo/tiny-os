use alloc::boxed::Box;
use spin::Mutex;
use logging::Logger;

use crate::logger::{logging::WriteSink, serial::writer::FALLBACK};
pub mod graphycal;
pub mod serial;
pub mod ring_buffer;

pub mod logging;

#[allow(dead_code)]
pub const DISPLAY_WRITER_ID:usize = 1;
#[allow(dead_code)]
pub const SERIAL_WRITER_ID:usize = 2;

pub static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());

#[macro_export]
macro_rules! panic_flush {
    () => {
        crate::logger::_panic_flush();
    };
}


#[macro_export]
macro_rules! flush {
    () => {{
        $crate::logger::_flush();
    }};
}


#[macro_export]
macro_rules! flush_all {
    () => {{
        $crate::logger::_flush_all();
    }};
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
pub fn _flush_all(){
    let mut log = LOGGER.lock();

    log.flush_all();
}


#[doc(hidden)]
pub fn _flush(){
    let mut log = LOGGER.lock();

    log.flush();
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

pub fn add_sink(sink: Box<dyn WriteSink + Send+ Sync>) {
    let mut logger = LOGGER.lock();

    logger.init_sink(sink);
}