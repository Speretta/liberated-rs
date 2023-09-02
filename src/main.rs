#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::panic::PanicInfo;


use terminal::*;
use vga::*;

mod terminal;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    terminal_initialize();
    terminal_write("hello", Color::White, Color::Black);

    loop {}
}

/// This function is called on panic.
#[panic_handler]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    clear_screen();
    unsafe {TERM_ROW = 0}
    
    let panic_msg = "
    \n\n
    A FATAL ERROR HAS OCCURRED IN THE OPERATING SYSTEM KERNEL,\n
    THE SYSTEM HAS TO BE STOPPED IN THIS CASE.\n
    WE RECOMMEND RESTARTING YOUR COMPUTER.\n
    IF THE PROBLEMS STILL PERSIST, YOU CAN OPEN A ISSUE IN GITHUB.
    \n\n
    
    !YOU CAN TURN OFF OR REBOOT YOUR COMPUTER!
    ";

    terminal_write(panic_msg, Color::Red, Color::Black);
    loop {}
}