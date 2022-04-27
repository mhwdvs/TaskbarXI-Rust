use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;

use crate::data::*;
use crate::window::*;

// top-level taskbar class name
static TASKBAR_CLASS: &str = "Shell_TrayWnd";
// top-level taskbar class name on other monitors
static SECONDARY_TASKBAR_CLASS: &str = "Shell_SecondaryTrayWnd";
// Windows notification tray (including system clock) class name
static TASKBAR_NOTIFICATION_TRAY_CLASS: &str = "TrayNotifyWnd";
static TASKBAR_APPLICATION_TRAY_CLASS: &str = "ReBarWindow32";

static RIGHT_NOTIFICATION_POP_OUT_CLASS: &str = "Windows.UI.Core.CoreWindow";
static RIGHT_NOTIFICATION_POP_OUT_NAME: &str = "Notification Centre";

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

pub fn set_taskbar() {
    // TODO: might need multiple tries + delay to find
    let taskbar = find_window(TASKBAR_CLASS);
    let window_region = get_window_region(taskbar);

    if window_region == ERROR.try_into().unwrap() {
        // in original, calls --restart on the service?
        panic!("Failed to get window region");
    }

    // clear maximized window list

    /*
    for (taskbar : taskbars)
    {
        // query registry "TaskbarAl" - determine if taskbar is centered

    }
    */
}

pub fn hide_taskbars() {
    // find taskbar processes
    find_taskbars();

    // hide taskbars
    unsafe {
        for taskbar in &TASKBARS {
            hide_window(*taskbar);
        }
    }
}

/// Finds all taskbars and their details
/// Details are appended to the TASKBARS
fn find_taskbars() -> bool {
    /**
     * Callback function for EnumWindows from the Windows API
     * Used to find all taskbars and store all of their details
     * Details are appended to the TASKBARS vec
     */
    unsafe extern "system" fn enum_windows_taskbars_callback(
        window_handle: HWND,
        _: LPARAM,
    ) -> BOOL {
        let is_taskbar = 0;
        let is_primary_taskbar = 0;

        // is primary taskbar
        if is_taskbar != 0 && is_primary_taskbar != 0 {
            println!("Main taskbar found! @ hwid: {:?}", window_handle);
            TASKBARS.push(window_handle);
        }
        // is regular taskbar
        else if is_taskbar != 0 && is_primary_taskbar == 0 {
            println!("Secondary taskbar found! @ hwid: {:?}", window_handle);
            TASKBARS.push(window_handle);
        } else {
            // stop enumeration
            return BOOL(true as i32);
        }

        // continue enumeration
        return BOOL(false as i32);
    }

    unsafe {
        let result = EnumWindows(Some(enum_windows_taskbars_callback), LPARAM(0));
        println!("Num taskbars found: {}", TASKBARS.len());
        if result == BOOL(0) {
            panic!("Failed to find taskbars");
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_taskbars() {
        assert_eq!(find_taskbars(), true);
    }
}
