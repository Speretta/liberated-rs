#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::{panic::PanicInfo, cell::OnceCell};

use terminal::Terminal;

mod terminal;
mod vga;

static mut TERMINAL: OnceCell<Terminal> = OnceCell::new();

pub fn get_and_init_terminal() -> &'static Terminal{
    unsafe {
        TERMINAL.get_or_init(|| terminal::Terminal::new())
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
    println!("{}", "deneme");
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

    println!("{} {}", panic_msg, info);
    loop {}
}