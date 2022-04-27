use windows::Win32::Foundation::HWND;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::UI::WindowsAndMessaging::GetClassNameA;

const WIN32_MAX_WINDOW_TITLE_LEN: usize = 256;

/// Similar to UNIX fork, moves process out of console context
/// When executed, a console will not be spawned
pub fn detach_from_console() {
    unsafe {
        FreeConsole();
    }
}

/// Uses Windows APIs to find and terminate any existing TaskbarXI processes
pub fn terminate_existing_processes() {
    unimplemented!();
}

pub fn get_class_name(window_handle: HWND) -> String {
    unsafe {
        // address of title
        let title_bytes: &mut [u8] = &mut [];
        // getclassnamew returns utf16
        let title_len = GetClassNameA(window_handle, title_bytes);

        if title_len == 0 || title_bytes.len() != title_len as usize {
            panic!("Failed to get window title");
        }

        let title = String::from_utf8(title_bytes.to_vec()).unwrap();

        if title.len() != title_len as usize {
            panic!("Failed to get window title");
        }

        return title;
    }
}
