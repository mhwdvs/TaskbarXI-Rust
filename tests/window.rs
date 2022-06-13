use taskbar11::window::*;

#[test]
fn integration_window_constructor_from_name() {
    _ = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
}

#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"Failed to find window\"")]
#[test]
fn window_constructor_from_name_invalid() {
    _ = Window::new_from_name(None, "", "").unwrap();
}

#[test]
fn integration_window_constructor_from_window_handle() {
    let w = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = Window::new_from_window_handle(w._window_handle).unwrap();
}

#[should_panic(
    expected = "called `Result::unwrap()` on an `Err` value: \"Failed to get window title\""
)]
#[test]
fn window_constructor_from_window_handle_invalid() {
    use windows::Win32::Foundation::HWND;
    _ = Window::new_from_window_handle(HWND(0)).unwrap();
}

#[test]
fn integration_window_main_taskbar() {
    _ = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
}

#[test]
fn integration_window_secondary_taskbars() {
    _ = Window::new_from_name(None, "", "Shell_SecondaryTrayWnd").unwrap();
}

#[test]
fn integration_window_main_taskbar_notif_tray() {
    // child of Shell_TrayWnd, use FindWindowExA
    let parent = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = Window::new_from_name(Some(parent._window_handle), "", "TrayNotifyWnd").unwrap();
}

#[test]
fn integration_window_main_taskbar_app_tray() {
    // child of Shell_TrayWnd, use FindWindowExA
    let parent = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = Window::new_from_name(Some(parent._window_handle), "", "ReBarWindow32").unwrap();
}

#[test]
fn integration_window_notification_panel() {
    _ = Window::new_from_name(None, "Notification Centre", "Windows.UI.Core.CoreWindow").unwrap();
}

#[test]
fn integration_window_get_region() {
    let w = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = get_region(w._window_handle).unwrap();
}

#[ignore]
#[test]
fn integration_window_hide() {
    let mut w = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = w.hide();
}

#[test]
fn integration_window_show() {}
