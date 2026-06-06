use crate::hal::framebuffer::{Framebuffer, Color};
use crate::logger::LOGGER;
use crate::logger::graphycal::bitmap_font::FONT_8X8;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct DisplayWriter {
    fb_ptr: *mut Framebuffer,

    padding: usize, 

    max_row: usize,
    max_col: usize,
    
    cur_row: usize,
    cur_col: usize,
    cell_size: u8,

    active_foreground: Color,
    active_background: Color
}


unsafe impl Sync for DisplayWriter {}

unsafe impl Send for DisplayWriter {}

impl Eq for DisplayWriter {}


impl PartialEq for DisplayWriter {
    fn eq(&self, other: &Self) -> bool {
        self.fb_ptr == other.fb_ptr
    }
}

#[allow(dead_code)]
impl  DisplayWriter {
    
    pub fn write_string(&mut self, string: &str){

        LOGGER.lock().write("DisplayWriter writing string...");
        
        for char  in string.chars() {
            self.write_symbol(char);
        }

        LOGGER.lock().write("DisplayWriter finished writing string");
    }

    pub fn write_symbol(&mut self, c: char) {
        if !c.is_ascii() {
            LOGGER.lock().write("[ERROR] expected ascii but it is not");
            return;
        }
        match c {
            '\n' => {
                self.new_line();
                return;
            },
            '\r' => {
                self.cur_col = self.padding;
                return;
            },
            _ => {

                let fb = unsafe { &*self.fb_ptr };

                let x_start = self.cur_col as usize * self.cell_size as usize;
                let y_start = self.cur_row as usize * self.cell_size as usize;

                fb.fill_rect(
                    self.active_background,
                    x_start,
                    y_start,
                    self.cell_size as usize, 
                    self.cell_size as usize
                );
                
                match c {
                    ' ' => {
                        // For space, we can just fill the cell with background color without drawing the symbol
                        self.calculate_offset_after_symbol();
                        return;
                    },      
                    _ => {
                        self.write_foreground(self.active_foreground, c, x_start, y_start, fb);
                    }
                }

                self.calculate_offset_after_symbol();
            }
        }        

    } 

    fn calculate_offset_after_symbol(&mut self) {
        if self.cur_col == self.max_col {
            self.new_line();
        } else {
            self.cur_col += 1;
        }
    }

    fn new_line(&mut self) {
        if self.cur_row >= self.max_row {
            todo!("Implement erasing first line and writing there second. Move cur to max") 
        }
        self.cur_row += 1;
        self.cur_col = self.padding;
    }

    fn write_foreground(&self, color: Color, c: char, x_start: usize, y_start: usize, fb: &Framebuffer) {

        let symbol = FONT_8X8[c as usize];

        for byte in 0..8 {
            for bit in 0..8{ 

                // we have ONE 1 therefore all other positions is zero. 
                // The & operation on other position will be zero but our shifted by bit "1" will be one on that position when symbol in bites will be one. 
                // Therefore whether zero or not depends on one position and we shift that position bites by bites and if it other than 0 it on required position 1. 
                // We draw a pixel on that position (shift starts on the cell by symbol. We have to shift 0..8 NOT in symbol because of shifting) 
                // I hope that I did not make any mistakes and explained it well.

                let x = symbol[byte] & (1 << bit);
                
                if x != 0 {
                        fb.write_pixel(
                            color, 
                            x_start  + bit as usize,
                            y_start  + byte as usize
                        );
                }
            }
        }       
    }
}

impl DisplayWriter {
    pub fn new(fb_ptr: *mut Framebuffer, padding: usize, apf: Color, apb: Color, cell_size: u8) -> Self{
        
        LOGGER.lock().write("DisplayWriter creating...");
        
        let width =  unsafe { (*fb_ptr).width  };
        let height = unsafe { (*fb_ptr).height };


        Self {
            fb_ptr,
            padding,
            max_row: (height as usize / (cell_size as usize)) as usize - padding,
            max_col: (width as usize / (cell_size as usize)) as usize - padding,
            cur_col: padding,
            cur_row: padding,
            active_background: apb,
            active_foreground: apf, 
            cell_size
        }
    }
}