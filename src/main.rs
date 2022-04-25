use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Gdi::CreateRectRgn;
use windows::Win32::Graphics::Gdi::SetWindowRgn;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::Win32::UI::WindowsAndMessaging::SendMessageW;
use windows::Win32::UI::WindowsAndMessaging::WM_THEMECHANGED;

fn main() {
    init();
}

fn init() {
    detach_from_console();

    // parse command line args

    println!("Initializing...");

    // register Windows API callbacks
    // terminate_existing_processes();

    // find explorer process

    // find taskbar processes

    // hide taskbars
    // for(taskbar : taskbars){
    //    hide_task_bar();
    //}

    println!("Initialize complete!");
}

/**
 * Similar to UNIX fork, moves process out of console context
 * When executed, a console will not be spawned
 */
fn detach_from_console() {
    unsafe {
        FreeConsole();
    }
}

/**
 * Uses Windows APIs to find and terminate any existing TaskbarXI processes
 */
fn terminate_existing_processes() {
    unimplemented!();
}

/**
 * Hides a Windows taskbar
 */
fn hide_task_bar(taskbar: HWND) {
    unsafe {
        let empty_region = CreateRectRgn(0, 0, 0, 0);
        if empty_region.is_invalid() {
            panic!("Winapi failed: CreateRectRgn");
        }
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
