#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub fn vga_entry_color(fg: Color, bg: Color) -> u8 {
    fg as u8 | (bg as u8) << 4
}

pub fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

pub const VGA_HEIGHT: usize = 25;
pub const VGA_WIDTH: usize = 80;