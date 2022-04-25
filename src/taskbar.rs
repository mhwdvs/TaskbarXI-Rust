use windows::Win32::Graphics::Gdi::*;

use crate::w11::*;

static TASKBAR_PROCESS_NAME: &str = "Shell_TrayWnd";

pub fn taskbar_loop() {
    loop {
        unimplemented!();

        // wait for message from system
        // getmessage

        // spawn thread in response to message
        std::thread::spawn(move || {
            set_taskbar();
        });
    }
}

fn set_taskbar() {
    // TODO: might need multiple tries + delay to find
    let taskbar = find_window(TASKBAR_PROCESS_NAME);
    let window_region = get_window_region(taskbar);

    if window_region == ERROR.try_into().unwrap() {
        // in original, calls --restart on the service?
        panic!("Failed to get window region");
    }

    // clear maximized window list

    // for (taskbar : taskbars)
}
