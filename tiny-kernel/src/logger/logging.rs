use core::fmt::Write;

use alloc::{boxed::Box, string::ToString, vec::{self, Vec}};

use crate::logger::ring_buffer::RingBuffer;

pub trait SinkEntity {
    fn get_id(&self) -> usize;
}

pub trait WriteSink: Write + SinkEntity {}

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

            let mut vector = Vec::new();

            vector.push(writer);

            self.sinks = Some(vector);

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

    pub fn flush(&mut self) {
        if let Some(sinks) = self.sinks.as_ref() {
            if sinks.is_empty() {
                return; 
            }
            
            // if vec is initialized, the dynamic memory will be accessible at that point and thus we CAN use to_string

            let mut buffer = [0u8;1000];
            let string = self.buffer.popln(&mut buffer).unwrap_or("");

            for sink in self.sinks.as_mut().unwrap() {
                let _ = sink.write_str(string);
            }
        } 
    }

    pub fn flush_all(&mut self) {
        if let Some(sinks) = self.sinks.as_ref() {
            if sinks.is_empty() {
                return; 
            }

            // if vec is initialized, the dynamic memory will be accessible at that point and thus we CAN use to_string
            
            
            loop {  
                
                let mut buffer = [0u8;1000];
                
                let string_curr= self.buffer.popln(&mut buffer);

                if let Some(string) = string_curr {
                    
                    for sink in self.sinks.as_mut().unwrap() {
                        let _ = sink.write_str(string);
                    }
                    
                } else {
                    return;
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