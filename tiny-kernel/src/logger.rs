
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

#[doc(hidden)]
pub fn _panic_flush(){

    let mut log = LOGGER.lock();
    log.write("Eror occured. Write after panic flush macros");
    if log.is_sinks_inited() {
        log.flush_all();
    } else {
        let mut line = log.read();
        while line.is_some() {
            FALLBACK.write_string(line.unwrap());
            line = log.read();
        }
    }
}