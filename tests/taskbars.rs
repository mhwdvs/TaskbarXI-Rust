use taskbar11::taskbars::*;

#[test]
fn integration_find_taskbars() {
    let ts = find_taskbars().unwrap();
    assert_eq!(ts._primary_taskbar._class, taskbar_constants::TASKBAR_CLASS);
    for st in ts._secondary_taskbars {
        assert_eq!(st._class, taskbar_constants::SECONDARY_TASKBAR_CLASS);
    }
}
