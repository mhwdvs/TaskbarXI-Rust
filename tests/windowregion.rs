use taskbar11::window::*;
use taskbar11::windowregion::*;

#[test]
fn integration_get_region() {
    let w = Window::new_from_name(None, "", "Shell_TrayWnd").unwrap();
    _ = get_region(w._window_handle).unwrap();
}
