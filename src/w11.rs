use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::UI::WindowsAndMessaging::*;

pub static mut TASKBARS: Vec<HWND> = Vec::new();

/**
 * Similar to UNIX fork, moves process out of console context
 * When executed, a console will not be spawned
 */
pub fn detach_from_console() {
    unsafe {
        FreeConsole();
    }
}

/**
 * Creates an empty (0x0) rectangle region
 */
pub fn empty_rect_rgn() -> HRGN {
    unsafe {
        let empty_region = CreateRectRgn(0, 0, 0, 0);
        if empty_region.is_invalid() {
            panic!("Winapi failed: CreateRectRgn");
        }
        return empty_region;
    }
}

/**
 * Gets the rectangle region currently occupied by a window
 */
pub fn get_window_region(window: HWND) -> i32 {
    unsafe {
        let temp = empty_rect_rgn();
        return GetWindowRgn(window, temp);
    }
}

pub fn set_window_region(window: HWND, new_region: HRGN) {
    unsafe {
        let set_window_rgn_result = SetWindowRgn(window, new_region, BOOL(true as i32));
        if set_window_rgn_result == 0 {
            panic!("Winapi failed: SetWindowRgn");
        }
    }
}

/**
 * Hides a Windows taskbar
 */
pub fn hide_taskbar(taskbar: HWND) {
    unsafe {
        // true redraws window after updating region
        set_window_region(taskbar, empty_rect_rgn());

        let send_message_result = SendMessageW(taskbar, WM_THEMECHANGED, WPARAM(0), LPARAM(0));
        if send_message_result == LRESULT(0) {
            panic!("Winapi failed: SendMessage WM_THEMECHANGED");
        }
    }
}

/**
 * Finds a window given its class name string
 */
pub fn find_window(window_name: &str) -> HWND {
    unsafe {
        return FindWindowW(window_name, PCWSTR(std::ptr::null()));
    }
}

/**
 * Finds all taskbars and their details
 * Details are appended to the TASKBARS
 */
pub fn find_taskbars() {
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
        }

        return BOOL(true as i32);
    }

    unsafe {
        EnumWindows(Some(enum_windows_taskbars_callback), LPARAM(0));
    }
}
