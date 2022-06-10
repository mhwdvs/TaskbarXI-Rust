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
