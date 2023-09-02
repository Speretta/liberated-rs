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
    let terminal = terminal::Terminal::new();

    

    loop {}
}

/// This function is called on panic.
#[panic_handler]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    
    let panic_msg = "
    \n\n
    A FATAL ERROR HAS OCCURRED IN THE OPERATING SYSTEM KERNEL,\n
    THE SYSTEM HAS TO BE STOPPED IN THIS CASE.\n
    WE RECOMMEND RESTARTING YOUR COMPUTER.\n
    IF THE PROBLEMS STILL PERSIST, YOU CAN OPEN A ISSUE IN GITHUB.
    \n\n
    
    !YOU CAN TURN OFF OR REBOOT YOUR COMPUTER!
    ";


    loop {}
}