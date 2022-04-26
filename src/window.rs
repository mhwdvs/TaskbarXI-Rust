use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/**
 * Gets the rectangle region currently occupied by a window
 */
pub fn get_window_region(window: HWND) -> i32 {
    unsafe {
        let temp = utility::empty_rect_rgn();
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
pub fn hide_window(window: HWND) {
    unsafe {
        // true redraws window after updating region
        set_window_region(window, utility::empty_rect_rgn());

        let send_message_result = SendMessageW(window, WM_THEMECHANGED, WPARAM(0), LPARAM(0));
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

mod utility {
    use windows::Win32::Graphics::Gdi::*;

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
}
