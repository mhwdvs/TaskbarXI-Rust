pub mod data;
pub mod taskbar;
pub mod w11;

use crate::taskbar::*;
use crate::w11::*;

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

    // terminate_existing_processes();

    // find explorer process

    // find taskbar processes

    // hide taskbars
    // for(taskbar : taskbars){
    //    hide_task_bar();
    //}

    println!("Initialized!");

    // spawn taskbar loop thread
    std::thread::spawn(move || {
        taskbar_loop();
    });
}

/**
 * Uses Windows APIs to find and terminate any existing TaskbarXI processes
 */
fn terminate_existing_processes() {
    unimplemented!();
}
