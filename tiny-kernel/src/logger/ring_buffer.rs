pub struct RingBuffer {
    buffer: [Option<&'static str>; 50],
    head: usize,
    tail: usize,
}


impl RingBuffer {

    pub const fn new() -> Self {
        Self {
            buffer: [None; 50],
         
            head:0,
         
            tail:0
        }
    }

    pub fn push(&mut self, data: &'static str) {
        
        self.buffer[self.head] = Some(data);

        self.head = (self.head + 1) % self.buffer.len();

        if self.head == self.tail {
            self.tail = (self.tail + 1) % self.buffer.len();
        }
    }

    pub fn pop(&mut self) -> Option<&'static str> {

        if self.tail == self.head {
            return None;
        }
        
        let data = self.buffer[self.tail];

        self.tail = (self.tail + 1) % self.buffer.len();

        data
    }

}