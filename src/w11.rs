use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::UI::WindowsAndMessaging::*;

/**
 * Similar to UNIX fork, moves process out of console context
 * When executed, a console will not be spawned
 */
pub fn detach_from_console() {
    unsafe {
        FreeConsole();
    }
}

pub fn empty_rect_rgn() -> HRGN {
    unsafe {
        let empty_region = CreateRectRgn(0, 0, 0, 0);
        if empty_region.is_invalid() {
            panic!("Winapi failed: CreateRectRgn");
        }
        return empty_region;
    }
}

pub fn get_window_region(window: HWND) -> i32 {
    unsafe {
        let temp = empty_rect_rgn();
        return GetWindowRgn(window, temp);
    }
}

/**
 * Hides a Windows taskbar
 */
pub fn hide_task_bar(taskbar: HWND) {
    unsafe {
        let empty_region = empty_rect_rgn();
        let set_window_rgn_result = SetWindowRgn(taskbar, empty_region, false);
        if set_window_rgn_result == 0 {
            panic!("Winapi failed: SetWindowRgn");
        }

        let send_message_result = SendMessageW(taskbar, WM_THEMECHANGED, WPARAM(0), LPARAM(0));
        if send_message_result == LRESULT(0) {
            panic!("Winapi failed: SendMessage WM_THEMECHANGED");
        }
    }
}

pub fn find_window(window_name: &str) -> HWND {
    unsafe {
        return FindWindowW(window_name, PCWSTR(std::ptr::null()));
    }
}
