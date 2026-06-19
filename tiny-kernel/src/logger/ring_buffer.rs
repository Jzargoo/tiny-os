use core::str::from_utf8;

pub struct RingBuffer {
    buffer: [u8; 64 * 1024],
    head: usize,
    tail: usize,
}

#[allow(dead_code)]
impl RingBuffer {

    pub const fn new() -> Self {
        Self {
            buffer: [0; 64 * 1024],
         
            head:0,
            tail:0
        }
    }

    pub fn push(&mut self, data: &'static str) {
        for byte in data.bytes() {
            self.push_byte(byte);
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        
        self.buffer[self.head] = byte;
        
        self.head = (self.head + 1) % self.buffer.len();
        
        if self.head == self.tail {
            self.tail = (self.tail + 1) % self.buffer.len();
        }

    }

    pub fn pop(&mut self) -> Option<u8> {

        if self.tail == self.head {
            return None;
        }
        
        let data = self.buffer[self.tail];

        self.tail = (self.tail + 1) % self.buffer.len();

        Some(data)
    }

    pub fn  popln<'a>(&mut self, buffer: &'a mut [u8]) -> Option<&'a str> {

        if self.head == self.tail {
            return None;
        }

        let mut idx = 0;

        while idx < buffer.len() {
            
            if let Some(byte) = self.pop() {
                
                buffer[idx] = byte;
                
                idx += 1;

                if byte == b'\n' {
                    break;
                }

            } else {
                break;
            }
        }

        if idx == 0 {
            return None;
        }

        from_utf8(&buffer[..idx]).ok()
    }

}