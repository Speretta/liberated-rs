use core::fmt::{self, Write};

use crate::{
    get_mut_terminal,
    vga::{vga_entry, Color, VGA_HEIGHT, VGA_WIDTH, vga_entry_color},
};

pub struct Terminal {
    position: (usize, usize),
    buffer: &'static mut [u16; VGA_WIDTH * VGA_HEIGHT],
}

impl Terminal {
    pub fn new() -> Self {
        let mut terminal = unsafe {
            Terminal {
                position: (0, 0),
                buffer: &mut *(0xB8000 as *mut [u16; VGA_WIDTH * VGA_HEIGHT]),
            }
        };
        terminal.clear_screen(Color::Black as u8);
        terminal
    }

    pub fn clear_screen(&mut self, color: u8) {
        let blank = vga_entry(b' ', color);
        for entry in self.buffer.iter_mut() {
            *entry = blank;
        }
        self.position.0 = 0;
        self.position.1 = 0;
    }

    fn terminal_putchar(&mut self, ch: char, color: u8) {
        if ch == '\n' || self.position.1 > VGA_WIDTH{
            self.position.0 += 1;
            self.position.1 = 0;
        }else if self.position.0 > VGA_HEIGHT{
            self.terminal_scroll(color);
        }
        self.buffer[self.position.0 * VGA_HEIGHT + self.position.1] = vga_entry(ch as u8, color);
        self.position.1 += 1;
    }

    fn terminal_scroll(&mut self, color: u8) {
        let tmp = &self.buffer.clone()[VGA_WIDTH..VGA_HEIGHT * VGA_WIDTH];
        for (i, x) in tmp.iter().enumerate() {
            self.buffer[i] = *x;
        }
        for i in (VGA_HEIGHT - 1) * VGA_WIDTH..VGA_HEIGHT * VGA_WIDTH {
            self.buffer[i] = vga_entry(b' ', color)
        }
        self.position.0 -= 1;
        self.position.1 = 0;
    }

    pub fn term_write_legacy(&mut self, data: &str, fg: Color, bg: Color) {
        let text_color: u8 = vga_entry_color(fg, bg);
        for char in data.chars() {
            self.terminal_putchar(char, text_color);
        }
    }
}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.terminal_putchar(ch, Color::White as u8);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    get_mut_terminal().write_fmt(args).unwrap();
}
