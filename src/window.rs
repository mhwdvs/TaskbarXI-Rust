use std::result::Result;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct Window {
    _caption: String,
    _class: String,
    _window_handle: HWND,
    _region_handle: HRGN,
}

impl Window {
    pub fn new(caption: &str, class: &str) -> Self {
        let window_handle = find_window_handle(caption, class).unwrap();
        let region_handle = create_region_handle().unwrap();

        return Self {
            _caption: caption.to_string(),
            _class: class.to_string(),
            _window_handle: window_handle,
            _region_handle: region_handle,
        };
    }

    /// Updates the rectangle region currently occupied by a window from Windows
    pub fn update_region(&self) -> Result<(), String> {
        unsafe {
            match GetWindowRgn(self._window_handle, self._region_handle) as u32 {
                NULLREGION | SIMPLEREGION | COMPLEXREGION => return Ok(()),
                ERROR => return Err("Failed to get window region".to_string()),
                _ => return Err("Unknown response".to_string()),
            }
        }
    }

    /// Sets the rectangle region currently occupied by a window
    pub fn set_region(&mut self, region: HRGN) -> Result<(), String> {
        unsafe {
            match SetWindowRgn(self._window_handle, region, BOOL(true as i32)) {
                _ => return Ok(()),
                0 => return Err("Failed to set window region".to_string()),
            }
        }
    }

    /// Hides a Windows taskbar
    pub fn hide(&mut self) -> Result<(), String> {
        // delete old handle, create new 0,0,0,0 handle
        delete_region(self._region_handle);
        self._region_handle = create_region_handle().unwrap();

        match self.set_region(self._region_handle) {
            Err(error) => return Err(error),
            _ => {}
        }
        unsafe {
            match SendMessageW(self._window_handle, WM_THEMECHANGED, WPARAM(0), LPARAM(0)) {
                LRESULT(0) => return Err("Failed to send WM_THEMECHANGED message".to_string()),
                _ => return Ok(()),
            }
        }
    }
}

/// Window destructor
impl Drop for Window {
    fn drop(&mut self) {
        delete_region(self._region_handle);
    }
}

/// Finds a window given its class name string
fn find_window_handle(class: &str, window: &str) -> Result<HWND, String> {
    let mut res: HWND;
    unsafe {
        // findwindoww is ANSI version of FindWindow API, A is unicode
        res = FindWindowW(class, window);
    }
    match res {
        HWND(0) => return Err("Failed to find window".to_string()),
        _ => return Ok(res),
    }
}

fn get_class_name(window_handle: HWND) -> Result<String, String> {
    unsafe {
        // address of title
        let title_bytes: &mut [u8] = &mut [];
        // getclassnamew returns utf16
        let title_len = GetClassNameA(window_handle, title_bytes);

        if title_len == 0 || title_bytes.len() != title_len as usize {
            return Err("Failed to get window title".to_string());
        }

        let title = String::from_utf8(title_bytes.to_vec()).unwrap();

        if title.len() != title_len as usize {
            return Err("Failed to get window title".to_string());
        }

        return Ok(title);
    }
}

pub fn register_window_resize_callbacks() {
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
        let title = get_class_name(hwnd).unwrap();

        if title == "MSTask" || title == "Toolba" {
            // trigger set_taskbar routine on below events
            // essentially overrides whatever changes the system tries to make to the taskbar
            todo!();
        }
    }

    fn set_win_event_hook(eventmin: u32, eventmax: u32) -> Result<HWINEVENTHOOK, String> {
        unsafe {
            let res = SetWinEventHook(
                eventmin,
                eventmax,
                HINSTANCE(0),
                Some(set_win_event_hook_callback),
                0,
                0,
                WINEVENT_SKIPOWNPROCESS,
            );
            match res {
                HWINEVENTHOOK(0) => return Err("Failed to set event hook".to_string()),
                _ => return Ok(res),
            }
        }
    }

    _ = set_win_event_hook(EVENT_SYSTEM_MOVESIZESTART, EVENT_SYSTEM_MOVESIZEEND);
    _ = set_win_event_hook(EVENT_OBJECT_CREATE, EVENT_OBJECT_DESTROY);
    _ = set_win_event_hook(EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MINIMIZEEND);
    _ = set_win_event_hook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND);
}

pub fn create_region_handle() -> Result<HRGN, String> {
    unsafe {
        let empty_region = CreateRectRgn(0, 0, 0, 0);
        if empty_region.is_invalid() {
            return Err("Failed to create rectangular region".to_string());
        }
        return Ok(empty_region);
    }
}

pub fn delete_region(region: HRGN) -> Result<(), String> {
    unsafe {
        // necessary, cant cast in match
        let boolfalse = BOOL(false as i32);
        match DeleteObject(region) {
            boolfalse => return Err("Failed to delete region".to_string()),
            _ => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_constructor() {
        Window::new("", "");
    }
}
