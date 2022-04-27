use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Gets the rectangle region currently occupied by a window
pub fn get_window_region(window: HWND) -> i32 {
    unsafe {
        let temp = utility::empty_rect_rgn();
        return GetWindowRgn(window, temp);
    }
}

/// Sets the rectangle region currently occupied by a window
pub fn set_window_region(window: HWND, new_region: HRGN) {
    unsafe {
        let set_window_rgn_result = SetWindowRgn(window, new_region, BOOL(true as i32));
        if set_window_rgn_result == 0 {
            panic!("Winapi failed: SetWindowRgn");
        }
    }
}

/// Hides a Windows taskbar
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

/// Finds a window given its class name string
pub fn find_window(window_name: &str) -> HWND {
    unsafe {
        return FindWindowW(window_name, PCWSTR(std::ptr::null()));
    }
}

pub fn register_window_resize_callbacks() {
    use super::*;
    use windows::Win32::UI::Accessibility::SetWinEventHook;
    use windows::Win32::UI::Accessibility::HWINEVENTHOOK;

    unsafe extern "system" fn set_win_event_hook_callback(
        hwineventhook: HWINEVENTHOOK,
        event: u32,
        hwnd: HWND,
        idobject: i32,
        idchild: i32,
        ideventthread: u32,
        dwmseventtime: u32,
    ) {
        // get class name of Window event corresponds to
        let title = processes::get_class_name(hwnd);

        if title == "MSTask" || title == "Toolba" {
            // trigger set_taskbar routine on below events
            taskbar::set_taskbar();
        }
    }

    fn set_win_event_hook(eventmin: u32, eventmax: u32) {
        unsafe {
            let result = SetWinEventHook(
                eventmin,
                eventmax,
                HINSTANCE(0),
                Some(set_win_event_hook_callback),
                0,
                0,
                WINEVENT_SKIPOWNPROCESS,
            );
        }
    }

    set_win_event_hook(EVENT_SYSTEM_MOVESIZESTART, EVENT_SYSTEM_MOVESIZEEND);
    set_win_event_hook(EVENT_OBJECT_CREATE, EVENT_OBJECT_DESTROY);
    set_win_event_hook(EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MINIMIZEEND);
    set_win_event_hook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND);

    //SetWinEventHook(EVENT_SYSTEM_MOVESIZESTART, EVENT_SYSTEM_MOVESIZEEND, NULL, WinEventProcCallback, 0, 0, WINEVENT_SKIPOWNPROCESS);
    //SetWinEventHook(EVENT_OBJECT_CREATE, EVENT_OBJECT_DESTROY, NULL, WinEventProcCallback, 0, 0, WINEVENT_SKIPOWNPROCESS);
    //SetWinEventHook(EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MINIMIZEEND, NULL, WinEventProcCallback, 0, 0, WINEVENT_SKIPOWNPROCESS);
    //SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, NULL, WinEventProcCallback, 0, 0, WINEVENT_SKIPOWNPROCESS);
}

/// Various window utility functions
mod utility {
    use windows::Win32::Graphics::Gdi::*;

    /// Creates an empty (0x0) rectangle region
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
