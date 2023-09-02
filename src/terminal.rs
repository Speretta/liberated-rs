

use core::fmt::Write;

use crate::vga::{VGA_WIDTH, vga_entry, Color, vga_entry_color, VGA_HEIGHT};



pub struct Terminal{
    position: (usize, usize),
    buffer: &'static mut [u16; VGA_WIDTH*VGA_HEIGHT],
}


impl Terminal{
    pub fn new() -> Self{
        let mut terminal = unsafe { Terminal { position: (0,0), buffer: &mut *(0xB8000 as *mut [u16; VGA_WIDTH*VGA_HEIGHT]) } };
        terminal.clear_screen(Color::Black as u8);
        terminal
    }

    fn clear_screen(&mut self, color: u8){
        for mut _entry in *self.buffer{
            _entry = vga_entry(' ' as u8, color);
        }
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

    fn terminal_scroll(&mut self, color: u8){
        let tmp = &self.buffer.clone()[VGA_WIDTH..VGA_HEIGHT*VGA_WIDTH];
        for (i, x) in tmp.into_iter().enumerate(){
            self.buffer[i] = *x;
        }
        for i in (VGA_HEIGHT-1)*VGA_WIDTH..VGA_HEIGHT*VGA_WIDTH{
            self.buffer[i] = vga_entry(' ' as u8, color)
        }
        self.position.0 -= 1;
        self.position.1 = 0;
        
    } 
}

impl Write for Terminal{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars(){
            self.terminal_putchar(ch, Color::White as u8);
        }
        Ok(())
    }
}

