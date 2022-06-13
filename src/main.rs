use taskbar11::*;

fn main() {
    init();

    loop {
        // Get system messages
        // PeekMessage
    }
}

fn init() {
    //processes::detach_from_console();
    // TODO: parse command line args
    //processes::terminate_existing_processes();

    // register Windows API callbacks
    window::register_window_resize_callbacks();

    // hide taskbars
    //taskbar::hide_taskbars();

    // spawn taskbar loop thread
    //std::thread::spawn(move || {
    //    taskbar::taskbar_loop();
    //});
}
