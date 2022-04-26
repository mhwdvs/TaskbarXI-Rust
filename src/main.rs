pub mod data;
pub mod processes;
pub mod taskbar;
pub mod window;

use crate::data::*;
use crate::processes::*;
use crate::taskbar::*;

fn main() {
    init();

    loop {
        // Get system messages
        // PeekMessage
    }
}

fn init() {
    detach_from_console();

    // parse command line args

    println!("Initializing...");

    // register Windows API callbacks

    terminate_existing_processes();

    // find explorer process

    // hide taskbars

    println!("Initialized!");

    // spawn taskbar loop thread
    std::thread::spawn(move || {
        taskbar_loop();
    });
}
