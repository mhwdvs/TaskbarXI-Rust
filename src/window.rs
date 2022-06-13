use std::result::Result;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct Window {
    pub _caption: String,
    pub _class: String,
    pub _window_handle: HWND,
    pub _region_handle: HRGN,
}

impl Window {
    pub fn new_from_name(
        parent_window: Option<HWND>,
        caption: &str,
        class: &str,
    ) -> Result<Self, String> {
        let window_handle = match find_window_handle(parent_window, caption, class) {
            Err(x) => return Err(x),
            Ok(x) => x,
        };
        let region_handle = match create_region_handle() {
            Err(x) => return Err(x),
            Ok(x) => x,
        };

        return Ok(Self {
            _caption: caption.to_string(),
            _class: class.to_string(),
            _window_handle: window_handle,
            _region_handle: region_handle,
        });
    }

    pub fn new_from_window_handle(window_handle: HWND) -> Result<Self, String> {
        let caption = match get_window_caption(window_handle) {
            Err(_x) => "".to_string(),
            Ok(x) => x,
        };
        let class = match get_window_class(window_handle) {
            Err(x) => return Err(x),
            Ok(x) => x,
        };
        let region_handle = match create_region_handle() {
            Err(x) => return Err(x),
            Ok(x) => x,
        };

        return Ok(Self {
            _caption: caption,
            _class: class,
            _window_handle: window_handle,
            _region_handle: region_handle,
        });
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
                0 => return Err("Failed to set window region".to_string()),
                _ => return Ok(()),
            }
        }
    }

    /// Hides a Windows taskbar
    pub fn hide(&mut self) -> Result<(), String> {
        // delete old handle, create new 0,0,0,0 handle
        _ = delete_region(self._region_handle);
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
        _ = delete_region(self._region_handle);
    }
}

/// Finds a window given its class name string
fn find_window_handle(
    parent_window: Option<HWND>,
    caption: &str,
    class: &str,
) -> Result<HWND, String> {
    let mut res: HWND;
    // call can be unreliable (Windows may create and destroy windows in background for optimisation)
    // by retrying theres a much better chance of success without causing infinite loop in case of bad input
    let found = false;
    let max_retries = 5;
    let mut retry_count = 0;
    while !found && retry_count < max_retries {
        unsafe {
            // FindWindowW is ANSI version of FindWindow API, A is unicode
            match parent_window {
                Some(x) => res = FindWindowExW(x, HWND(0), class, caption),
                None => res = FindWindowExW(HWND(0), HWND(0), class, caption),
            }
        }
        match res {
            HWND(0) => retry_count += 1,
            _ => return Ok(res),
        }
    }
    return Err("Failed to find window".to_string());
}

fn get_window_class(window_handle: HWND) -> Result<String, String> {
    let title_len = 120usize;
    // \0 isnt considered whitespace
    let mut title_vec: Vec<u8> = vec![' ' as u8; title_len];
    unsafe {
        let res = GetClassNameA(window_handle, title_vec.as_mut_slice());
        let title = String::from_utf8(title_vec)
            .unwrap()
            .trim()
            .trim_end_matches('\u{0}')
            .to_string();

        if res == 0 || title_len < res as usize || title.len() != res as usize {
            return Err("Failed to get window title".to_string());
        }
        return Ok(title);
    }
}

fn get_window_caption(window_handle: HWND) -> Result<String, String> {
    unsafe {
        let title_len = GetWindowTextLengthA(window_handle);
        let mut title_vec = vec![0; title_len as usize + 1];
        let res = GetWindowTextA(window_handle, title_vec.as_mut_slice());

        let title = String::from_utf8(title_vec)
            .unwrap()
            .trim()
            .trim_end_matches('\u{0}')
            .to_string();

        if res == 0 || title_len != res || title.len() != res as usize {
            return Err("Failed to get window title".to_string());
        }

        return Ok(title);
    }
}

pub fn register_window_resize_callbacks() {
    unsafe extern "system" fn set_win_event_hook_callback(
        _hwineventhook: HWINEVENTHOOK,
        _event: u32,
        hwnd: HWND,
        _idobject: i32,
        _idchild: i32,
        _ideventthread: u32,
        _dwmseventtime: u32,
    ) {
        // get class name of Window event corresponds to
        let title = get_window_class(hwnd).unwrap();

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
        match DeleteObject(region) {
            BOOL(0) => return Err("Failed to delete region".to_string()),
            _ => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_get_caption() {
        let w = Window::new_from_name(None, "Notification Centre", "Windows.UI.Core.CoreWindow")
            .unwrap();
        assert_eq!(
            "Notification Centre".to_string(),
            get_window_caption(w._window_handle).unwrap()
        );
    }

    #[test]
    fn window_get_class() {
        let w = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
        assert_eq!("Shell_TrayWnd", get_window_class(w._window_handle).unwrap());
    }
}
