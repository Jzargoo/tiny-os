pub mod framebuffer;

pub mod buddy_mem_manager;

pub mod kernel_allocator;

pub mod bios_info;

pub mod page_allocator;

pub const KERNEL_HEAP_SIZE: usize = 1024 * 1024 * 4; // 2MB

use framebuffer::Color;

#[allow(dead_code)]
pub static BLACK:Color = Color::from_hex_rgb(0x0);

#[allow(dead_code)]
pub static WHITE:Color = Color::from_hex_rgb(0xFFFFFF);

#[allow(dead_code)]
pub static RED:Color = Color::from_hex_rgb(0xFF0000);

#[allow(dead_code)]
pub static BLUE:Color =Color::from_hex_rgb(0x0000FF);

#[allow(dead_code)]
pub static GREEN:Color = Color::from_hex_rgb(0x00FF00);

#[allow(dead_code)]
pub static YELLOW: Color = Color::from_hex_rgb(0xFFFF00);

#[allow(dead_code)]
pub static MAGENTA: Color = Color::from_hex_rgb(0xFF00FF);

#[allow(dead_code)]
pub static CYAN: Color = Color::from_hex_rgb(0x00FFFF);

#[allow(dead_code)]
pub static ORANGE: Color = Color::from_hex_rgb(0xFFA500);

#[allow(dead_code)]
pub static INDIGO: Color = Color::from_hex_rgb(0x4B0082);

#[allow(dead_code)]
pub static VIOLET: Color = Color::from_hex_rgb(0xEE82EE);

#[allow(dead_code)]
pub static PURPLE: Color = Color::from_hex_rgb(0x800080);

#[allow(dead_code)]
pub static PINK: Color = Color::from_hex_rgb(0xFFC0CB);

pub const RAINBOW: [Color; 7] = [RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET];
