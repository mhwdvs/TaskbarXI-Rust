use windows::Win32::System::Console::FreeConsole;

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
