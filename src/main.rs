#![no_main]
#![cfg_attr(not(test), no_std)]
#![feature(panic_info_message)]

use core::{panic::PanicInfo, cell::OnceCell};

mod terminal;
mod vga;

mod tests;

use terminal::Terminal;
use crate::vga::Color;

static mut TERMINAL: OnceCell<Terminal> = OnceCell::new();

pub fn get_and_init_terminal() -> &'static Terminal{
    unsafe {
        TERMINAL.get_or_init(terminal::Terminal::new)
    }
}

pub fn get_mut_terminal() -> &'static mut Terminal{
    unsafe {
        TERMINAL.get_mut().unwrap()
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    get_and_init_terminal();
    loop {}
}


#[panic_handler]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    get_mut_terminal().clear_screen(Color::Black as u8);

    let def_panic_msg = "
    \n\n
    A FATAL ERROR HAS OCCURRED IN THE OPERATING SYSTEM KERNEL,\n
    THE SYSTEM HAS TO BE STOPPED IN THIS CASE.\n
    WE RECOMMEND RESTARTING YOUR COMPUTER.\n
    IF THE PROBLEMS STILL PERSIST, YOU CAN OPEN A ISSUE IN GITHUB.
    \n\n
    
    !YOU CAN TURN OFF OR REBOOT YOUR COMPUTER!
    ";

    get_mut_terminal().term_write_legacy(def_panic_msg, Color::Red, Color::Black);
    println!("\n\nSome useful informations about error:\n\n{}", info);
    loop {}
}