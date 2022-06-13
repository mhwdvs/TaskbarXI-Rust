use crate::window::*;
use std::result::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;

pub mod taskbar_constants {
    // top-level taskbar class name
    pub static TASKBAR_CAPTION: &str = "";
    pub static TASKBAR_CLASS: &str = "Shell_TrayWnd";
    // top-level taskbar class name on other monitors
    pub static SECONDARY_TASKBAR_CAPTION: &str = "";
    pub static SECONDARY_TASKBAR_CLASS: &str = "Shell_SecondaryTrayWnd";
    // Windows notification tray (including system clock) class name
    pub static TASKBAR_NOTIFICATION_TRAY_CAPTION: &str = "";
    pub static TASKBAR_NOTIFICATION_TRAY_CLASS: &str = "TrayNotifyWnd";

    pub static TASKBAR_APPLICATION_TRAY_CAPTION: &str = "";
    pub static TASKBAR_APPLICATION_TRAY_CLASS: &str = "ReBarWindow32";

    pub static RIGHT_NOTIFICATION_POP_OUT_CAPTION: &str = "Notification Centre";
    pub static RIGHT_NOTIFICATION_POP_OUT_CLASS: &str = "Windows.UI.Core.CoreWindow";
}

static mut PRIMARY_TASKBAR: Option<Window> = None;
static mut SECONDARY_TASKBARS: Vec<Window> = Vec::new();

pub struct Taskbars {
    pub _primary_taskbar: Window,
    pub _secondary_taskbars: Vec<Window>,
}

impl Taskbars {
    pub fn new(primary_taskbar: Window, secondary_taskbars: Vec<Window>) -> Taskbars {
        return Taskbars {
            _primary_taskbar: primary_taskbar,
            _secondary_taskbars: secondary_taskbars,
        };
    }
}

//pub fn taskbar_loop() {
//    loop {
//        todo!();
//
//        // wait for message from system
//        // getmessage
//
//        // spawn thread in response to message
//        std::thread::spawn(move || {
//            set_taskbar();
//        });
//    }
//}

/// Finds all taskbars and their details
pub fn find_taskbars() -> Result<Taskbars, String> {
    /**
     * Callback function for EnumWindows from the Windows API
     * Used to find all taskbars and store all of their details
     * Details are appended to the TASKBARS vec
     */
    unsafe extern "system" fn enum_windows_taskbars_callback(
        window_handle: HWND,
        _: LPARAM,
    ) -> BOOL {
        let w = match Window::new_from_window_handle(window_handle) {
            Ok(x) => x,
            Err(_x) => {
                return BOOL(true as i32);
            }
        };
        match w._class.as_str() {
            "Shell_TrayWnd" => {
                // is primary taskbar
                PRIMARY_TASKBAR = Some(w);
            }
            "Shell_SecondaryTrayWnd" => {
                // is regular taskbar
                SECONDARY_TASKBARS.push(w);
            }
            _ => {}
        }

        // enumerate through all windows
        return BOOL(true as i32);
    }

    unsafe {
        // re-initialise static variables
        PRIMARY_TASKBAR = None;
        SECONDARY_TASKBARS.clear();

        match EnumWindows(Some(enum_windows_taskbars_callback), LPARAM(0)) {
            BOOL(0) => return Err("Failed to find taskbars".to_string()),
            _ => {
                return Ok(Taskbars::new(
                    PRIMARY_TASKBAR.clone().unwrap(),
                    SECONDARY_TASKBARS.clone(),
                ))
            }
        }
    }
}

//pub fn set_taskbar() {
//    // TODO: might need multiple tries + delay to find
//    let taskbar = find_window(TaskbarConstants::TASKBAR_CLASS);
//    let window_region = get_window_region(taskbar);
//
//    if window_region == ERROR.try_into().unwrap() {
//        // in original, calls --restart on the service?
//        panic!("Failed to get window region");
//    }
//
//    // clear maximized window list
//
//    /*
//    for (taskbar : taskbars)
//    {
//        // query registry "TaskbarAl" - determine if taskbar is centered
//
//    }
//    */
//}

//pub fn hide_taskbars() {
//    // find taskbar processes
//    find_taskbars();
//
//    // hide taskbars
//    unsafe {
//        for taskbar in &TASKBARS {
//            todo!();
//            *taskbar.hide();
//            hide_window(*taskbar);
//        }
//    }
//}
