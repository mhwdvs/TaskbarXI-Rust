use crate::window::*;
use std::ops;
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

#[derive(Clone)]
pub struct Taskbars {
    pub _primary_taskbar: Window,
    pub _secondary_taskbars: Vec<Window>,
}

pub struct TaskbarsIter<'a> {
    _taskbars: &'a Taskbars,
    _index: usize,
}

impl Taskbars {
    pub fn new(primary_taskbar: Window, secondary_taskbars: Vec<Window>) -> Taskbars {
        return Taskbars {
            _primary_taskbar: primary_taskbar,
            _secondary_taskbars: secondary_taskbars,
        };
    }

    pub fn iter(&self) -> TaskbarsIter {
        TaskbarsIter {
            _taskbars: self,
            _index: 0,
        }
    }

    pub fn count(&self) -> usize {
        return self._secondary_taskbars.len() + 1;
    }
}

impl<'a> Iterator for TaskbarsIter<'a> {
    type Item = &'a Window;

    fn next(&mut self) -> Option<Self::Item> {
        if self._index < self._taskbars.count() {
            self._index += 1;
            return Some(&self._taskbars[self._index - 1]);
        } else {
            return None;
        }
    }
}

impl ops::Index<usize> for Taskbars {
    type Output = Window;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => return &self._primary_taskbar,
            _ => {
                if index - 1 < self._secondary_taskbars.len() {
                    return &self._secondary_taskbars[index - 1];
                } else {
                    panic!("Index out of bounds");
                }
            }
        }
    }
}

pub fn taskbar_loop() {
    loop {
        // wait for message from system
        // getmessage

        // spawn thread in response to message
        std::thread::spawn(move || {
            set_taskbar();
        });
    }
}

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

pub fn reset_taskbars(tbs: Taskbars) {
    for tb in tbs._secondary_taskbars {
        tb.redraw();
    }
}

pub fn set_taskbar() {
    // TODO: might need multiple tries + delay to find
    let _tbs = find_taskbars().unwrap();

    /*
    for (taskbar : taskbars)
    {
        // query registry "TaskbarAl" - determine if taskbar is centered

    }
    */
}

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
