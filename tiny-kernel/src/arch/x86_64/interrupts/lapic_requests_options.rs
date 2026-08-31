pub struct TimerOptions{
    timer_mode: u8,
    vector: u8,
    delivery_status: u8,
    mask: u8
}

impl TimerOptions{ 

    pub fn get_timer_mode(&self) -> u8{
        self.timer_mode & 0b11
    }

    pub fn get_mask(&self) -> u8{
        self.mask & 0b1
    }
    
    pub fn get_vector(&self) -> u8{
        self.vector
    }
    
    pub fn get_delivery_status(&self) -> u8{
        self.delivery_status & 0b1
    }
    
    pub fn new(mask: u8, delivery_status: u8, vector: u8, timer_mode: u8) -> Self{
        
        Self {
            mask, delivery_status,  vector,  timer_mode
        }

    }

}

