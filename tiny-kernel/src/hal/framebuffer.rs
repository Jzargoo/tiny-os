use core::ptr::{copy_nonoverlapping, slice_from_raw_parts_mut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub const fn from_hex_rgb(hex_code: u32) -> Self {
        Self {
            r: ((hex_code >> 16) & 0xFF) as u8,
            g: ((hex_code >> 8) & 0xFF) as u8,
            b: (hex_code & 0xFF) as u8,
        }
    }
}

pub enum PixelFormat {
    RGBA, RBGA ,BGRA, BRGA, GBRA, GRBA, UNKNOWN
}

#[derive(Debug, Clone, Copy)]
pub struct Framebuffer {
    pub start_addr: *mut (),
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub red_mask_shift: u8,
    pub green_mask_shift: u8,
    pub blue_mask_shift: u8,
    pub red_mask_size: u8,
    pub green_mask_size: u8,
    pub blue_mask_size: u8,
    pub bpp: u16
}

impl Framebuffer {
    pub fn new (
                    start_addr:*mut (), width: u64, height: u64,pitch: u64,
                    red_mask_size: u8, green_mask_size: u8, blue_mask_size: u8,
                    red_mask_shift: u8, green_mask_shift: u8, blue_mask_shift: u8,
                    bpp: u16
                ) -> Self{
        Self {
            start_addr,
            width,
            height,
            pitch,
            red_mask_shift, green_mask_shift, blue_mask_shift,
            red_mask_size, green_mask_size, blue_mask_size,
            bpp
        }
    } 

    pub fn pixel_format(&self) -> PixelFormat {
        if self.bpp != 4 {
            return PixelFormat::UNKNOWN;
        }
        match (self.blue_mask_shift, self.green_mask_shift, self.red_mask_shift) {
            (0 , 8 , 16) => PixelFormat::BGRA,
            (0 , 16,  8) => PixelFormat::BRGA,
            (16, 8 ,  0) => PixelFormat::RGBA,
            (8 , 16,  0) => PixelFormat::RBGA,
            (16, 0 ,  8) => PixelFormat::GRBA,
            (8 , 0 , 16) => PixelFormat::GBRA,
            _ => PixelFormat::UNKNOWN,
        }
    }

    pub fn fill_rect(&self, color: Color, x_start:usize, y_start:usize, width: usize, height: usize) {
        // Ensure that the starting coordinates are within the framebuffer boundaries
        if x_start >= self.width as usize || y_start >= self.height as usize {
            return;
        }

        // Ensure that the rectangle does not exceed the framebuffer boundaries
        let x_end = (x_start + width).min(self.width as usize);
        let y_end = (y_start + height).min(self.height as usize);

        // Convert the Color struct to the raw color format used by the framebuffer
        let raw_color = self.convert_color(color);

        for y in y_start..y_end {

            // Calculate the starting address for the current row
            let row_offset = (y * self.pitch as usize) + (x_start * self.bpp as usize);
            
            unsafe {
                
                let mut pixel_ptr = (self.start_addr as *mut u8).add(row_offset);

                // Optimization: if bpp is 4, we can write the color directly as u32, otherwise we need to write byte by byte
                if self.bpp as usize == 4 {
                    
                    let pixel_slice = slice_from_raw_parts_mut(pixel_ptr as *mut u32, x_end - x_start);
                    
                    for pixel in &mut *pixel_slice {
                        *pixel = raw_color as u32;
                    }

                } else  {
                    
                    for _ in x_start..x_end {                    
                    
                        copy_nonoverlapping(
                            &raw_color as *const usize as *const u8, 
                            pixel_ptr,
                             self.bpp as usize
                        );

                        pixel_ptr = pixel_ptr.add(self.bpp as usize);

                    } 

                }
            }
        }
    }


    pub fn write_pixel(&self,color: Color, x:usize, y:usize) {
        
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        let index = (y * self.pitch as usize) + (x * self.bpp as usize);

        let raw_color = self.convert_color(color);

        unsafe {
            let pixel_ptr = (self.start_addr as *mut u8).add(index);
            // Optimization: if bpp is 4, we can write the color directly as u32, otherwise we need to write byte by byte
            if self.bpp as usize == 4 {
                let pixel_ptr = pixel_ptr as *mut u32;
                *pixel_ptr = raw_color as u32;
            } else {
                for i in 0..self.bpp as usize {
                    let byte = (raw_color >> (i * 8)) & 0xFF;
                    *pixel_ptr.add(i) = byte as u8;
                }
            }   
        }
    }

    fn convert_color(&self, color: Color) -> usize {
        let r_downshift = 8usize.saturating_sub(self.red_mask_size as usize);
        let g_downshift = 8usize.saturating_sub(self.green_mask_size as usize);
        let b_downshift = 8usize.saturating_sub(self.blue_mask_size as usize);

        let r = ((color.r as usize >> r_downshift) & ((1 << self.red_mask_size) - 1)) << self.red_mask_shift;
        let g = ((color.g as usize >> g_downshift) & ((1 << self.green_mask_size) - 1)) << self.green_mask_shift;
        let b = ((color.b as usize >> b_downshift) & ((1 << self.blue_mask_size) - 1)) << self.blue_mask_shift;

        r | g | b
    }

}