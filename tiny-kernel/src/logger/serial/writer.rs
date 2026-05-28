use core::{arch::asm, fmt::Write};

use lazy_static::lazy_static;

const COM_PORT: u16 = 0x3F8;

lazy_static!{
    pub static ref FALLBACK: SerialWriter = {
        initialization(COM_PORT);
        
        SerialWriter{
            port: COM_PORT
        }
    };
}
    
fn outb(com_port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") com_port,
            in("al") value,
            options(nomem, preserves_flags, nostack)
        );
    }
}

fn inb(com_port: u16) -> u8 {
    let value;

    unsafe{
        asm!(
            "in al, dx",
            in("dx") com_port,
            out("al") value,
            options(nomem, preserves_flags, nostack)
        )
    }

    value
}

pub fn initialization(port: u16) -> Option<()>{
    
    outb(port + 2, 0x00); //Disable all interruptions

    outb(port + 3, 0x80); // Enable DAHL

    outb(port + 0, 0x03); // Denum: 3

    outb(port + 1, 0x00);
    
    outb(port + 3, 0x03); // Disable DAHL 1 stop bit, no parity

    outb(port + 2, 0xC7); // set to 14, enable, and clear FIFO  

    outb(port + 4, 0x0B); // enable lookup

    outb(port + 0, 0xB2); // Send data

    if inb(port + 0) != 0xB2 {
        return None;
    }

    outb(port + 4, 0x0F);    

    Some(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SerialWriter {
    port: u16
}

impl SerialWriter{
    pub fn write_string(&self,string: &str){
        for byte in string.bytes() {
            outb(self.port, byte);
        }
    }
}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

unsafe impl Sync for SerialWriter {}
unsafe impl Send for SerialWriter {}
