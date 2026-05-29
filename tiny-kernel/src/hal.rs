pub mod framebuffer;
pub mod page_allocator;
pub mod bios_info;

use framebuffer::Color;

#[allow(dead_code)]
pub static BLACK:Color = Color::from_hex_rgb(0x0);

#[allow(dead_code)]
pub static WHITE:Color = Color::from_hex_rgb(0xFFFFFF);

#[allow(dead_code)]
pub static RED:Color = Color::from_hex_rgb(0x000000FF);

#[allow(dead_code)]
pub static BLUE:Color =Color::from_hex_rgb(0xFF0000);

#[allow(dead_code)]
pub static GREEN:Color = Color::from_hex_rgb(0x00FF00);

#[allow(dead_code)]
pub static YELLOW:Color = Color::from_hex_rgb(0x00FFFF);

#[allow(dead_code)]
pub static MAGENTA: Color = Color::from_hex_rgb(0xFF00FF);

#[allow(dead_code)]
pub static CYAN: Color = Color::from_hex_rgb(0x00FFFF);