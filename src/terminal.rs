use crate::vga::{VGA_WIDTH, vga_entry, Color, vga_entry_color, VGA_HEIGHT};

static mut TERMINAL_BUFFER: *mut u16 = 0xB8000 as *mut u16;
pub static mut TERM_ROW: usize = 0;
static mut TERM_COL: usize = 0;

pub fn clear_screen() {
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            let index: usize = y* VGA_WIDTH + x;
            unsafe {
                *TERMINAL_BUFFER.offset((index as usize).try_into().unwrap()) = vga_entry(' ' as u8, Color::Black as u8);
            }
        }
    }
}

fn terminal_scroll(color: u8) {
    unsafe {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let src_index = y * VGA_WIDTH + x;
                let dest_index = (y - 1) * VGA_WIDTH + x;
                *TERMINAL_BUFFER.offset(dest_index as isize) =
                        *TERMINAL_BUFFER.offset(src_index as isize);
            }
        }

        for x in 0..VGA_WIDTH {
            let index = (VGA_HEIGHT - 1) * VGA_WIDTH + x;
                *TERMINAL_BUFFER.offset(index as isize) =
                    vga_entry(' ' as u8, color);
        }

        TERM_ROW = VGA_HEIGHT - 1;
        TERM_COL = 0;
    }
}

fn terminal_putentryat(c: char, color: u8, x: usize, y: usize) {
    let index: usize = y * VGA_WIDTH + x;
    unsafe {
        *TERMINAL_BUFFER.offset((index as usize).try_into().unwrap()) = vga_entry(c as u8, color)
    }
}

fn terminal_putchar(c: char, color: u8) {
    unsafe {
        if c == '\n' {
            TERM_ROW += 1;
            TERM_COL = 0;
        }
        
        else {
            terminal_putentryat(c, color, TERM_COL, TERM_ROW);
            TERM_COL += 1;

            // if reached to end of the row.
            if TERM_COL >= VGA_WIDTH {
                TERM_ROW += 1;
                TERM_COL = 0;
            }

            // if reached to end of the screen.
            if TERM_ROW >= VGA_HEIGHT {
                terminal_scroll(color);
            }
        }
    }
}

pub fn terminal_write(data: &str, fg: Color, bg: Color) {
    let text_color: u8 = vga_entry_color(fg, bg);
    for char in data.chars() {
        terminal_putchar(char, text_color);
    }
}

pub fn terminal_initialize() {
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            let index: usize = y* VGA_WIDTH + x;
            unsafe {
                *TERMINAL_BUFFER.offset((index as usize).try_into().unwrap()) = vga_entry(' ' as u8, Color::Black as u8);
            }
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::terminal_putchar(format_args!($($arg)*), 0x13));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}