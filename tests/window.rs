use taskbar11::window::Window;

#[test]
fn integration_main_taskbar() {
    _ = Window::new(None, "", "Shell_TrayWnd").unwrap();
}

#[test]
fn integration_secondary_taskbars() {
    _ = Window::new(None, "", "Shell_SecondaryTrayWnd").unwrap();
}

#[test]
fn integration_main_taskbar_notif_tray() {
    // child of Shell_TrayWnd, use FindWindowExA
    let parent = Window::new(None, "", "Shell_TrayWnd").unwrap();
    _ = Window::new(Some(parent._window_handle), "", "TrayNotifyWnd").unwrap();
}

#[test]
fn integration_main_taskbar_app_tray() {
    // child of Shell_TrayWnd, use FindWindowExA
    let parent = Window::new(None, "", "Shell_TrayWnd").unwrap();
    _ = Window::new(Some(parent._window_handle), "", "ReBarWindow32").unwrap();
}

#[test]
fn integration_notification_panel() {
    _ = Window::new(None, "Notification Centre", "Windows.UI.Core.CoreWindow").unwrap();
}

#[test]
fn integration_window_valid_region() {
    let w = Window::new(None, "", "Shell_TrayWnd").unwrap();
    // checks status of region
    _ = w.update_region().unwrap();
}
