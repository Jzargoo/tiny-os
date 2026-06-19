use core::fmt::Write;

use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::logger::ring_buffer::RingBuffer;

trait SinkEntity {
    fn get_id(&self) -> usize;
}

trait WriteSink: Write + SinkEntity {}

pub struct  Logger {
    sinks: Option<
            Vec <
                Box<dyn WriteSink + Send + Sync>
            >
        >,
    buffer: RingBuffer,
    scratch_buffer: [u8; 1024], 
}

impl Logger {
    
    pub const fn new()-> Self{
        Logger {
            sinks: None,
            buffer: RingBuffer::new(),
            scratch_buffer: [0; 1024]
        }
    }

    pub fn read(&mut self) ->Option<&str> {
        self.buffer.popln(&mut self.scratch_buffer)
    }


    pub fn is_sinks_inited(&self)-> bool{
        self.sinks.is_some()
    }

    pub fn init_sink(&mut self, writer:Box<dyn WriteSink+ Send + Sync>) {

        if self.sinks.is_none()  {
            return;
        }

        self.sinks
            .as_mut()
            .unwrap()
            .push(writer);
    }

    pub fn remove_sink(&mut self, sink_id: usize) {
        
        if let Some(vec) = self.sinks.as_mut() {
            vec.retain(|sink| sink.get_id() != sink_id);
        }
    }

    pub fn extract_sink(&mut self, sink_id: usize) -> Option<Box<dyn WriteSink>> {
        let vec= self.sinks.as_mut()?;

        let pos = vec.iter().position(|sink| sink.get_id() == sink_id)?;

        Some(vec.remove(pos))
    }

}

impl Logger {
    
    fn push_byte(&mut self, char: u8){
        self.buffer.push_byte(char);
    }

    fn pushln(&mut self, data: &'static str){
        self.buffer.push(data);
        self.buffer.push("\n\r");
    }


    pub fn flush(&mut self) {
        if let Some(sinks) = self.sinks.as_ref() {
            if sinks.is_empty() {
                return; 
            }
        } 

        // if vec is initialized, the dynamic memory will be accessible at that point and thus we CAN use to_string

        if let Some(line_string) = self.read().map(|s| s.to_string()) {

            if let Some(sinks) = self.sinks.as_mut() {

                for sink in sinks.iter_mut() {
                    let _ = sink.write_str(&line_string);
                }

            }

        }
    }

    pub fn flush_all(&mut self) {
        if let Some(sinks) = self.sinks.as_ref() {
            if sinks.is_empty() {
                return; 
            }
        } 

        // if vec is initialized, the dynamic memory will be accessible at that point and thus we CAN use to_string

        while let Some(line_string) = self.read().map(|s| s.to_string()) {

            if let Some(sinks) = self.sinks.as_mut() {

                for sink in sinks.iter_mut() {
                    let _ = sink.write_str(&line_string);
                }

            }

        }
    }

}

impl Write for Logger{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        
        for byte in s.bytes() {
            self.push_byte(byte);
        }
        
        Ok(())
    }
}