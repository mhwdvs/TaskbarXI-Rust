use windows::Win32::System::Console::FreeConsole;

fn main() {
    detach_from_console();
}

fn detach_from_console() {
    unsafe {
        FreeConsole();
    }
}
