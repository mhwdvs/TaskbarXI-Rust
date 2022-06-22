use taskbar11::taskbars::*;

#[test]
fn integration_find_taskbars() {
    let ts = find_taskbars().unwrap();
    assert_eq!(ts._primary_taskbar._class, taskbar_constants::TASKBAR_CLASS);
    for st in &ts._secondary_taskbars {
        assert_eq!(st._class, taskbar_constants::SECONDARY_TASKBAR_CLASS);
    }
}

#[test]
fn taskbars_iterator() {
    let tbs = find_taskbars().unwrap();

    let mut count: usize = 0;
    for tb in tbs.iter() {
        // check that contents gathered via iterator are idential to what we would expect from using the base item
        match count {
            0 => {
                assert_eq!(tbs._primary_taskbar._caption, tb._caption);
                assert_eq!(tbs._primary_taskbar._class, tb._class);
                assert_eq!(tbs._primary_taskbar._window_handle, tb._window_handle);
            }
            _ => {
                assert_eq!(tbs._secondary_taskbars[count - 1]._caption, tb._caption);
                assert_eq!(tbs._secondary_taskbars[count - 1]._class, tb._class);
                assert_eq!(
                    tbs._secondary_taskbars[count - 1]._window_handle,
                    tb._window_handle
                );
            }
        }
        count += 1;
    }
    assert_eq!(tbs._secondary_taskbars.len() + 1, count);
}

#[test]
fn taskbars_index() {
    let tbs = find_taskbars().unwrap();

    for index in 0..tbs.count() - 1 {
        // check that contents gathered via iterator are idential to what we would expect from using the base item
        match index {
            0 => {
                assert_eq!(tbs._primary_taskbar._caption, tbs[index]._caption);
                assert_eq!(tbs._primary_taskbar._class, tbs[index]._class);
                assert_eq!(
                    tbs._primary_taskbar._window_handle,
                    tbs[index]._window_handle
                );
            }
            _ => {
                assert_eq!(
                    tbs._secondary_taskbars[index - 1]._caption,
                    tbs[index]._caption
                );
                assert_eq!(tbs._secondary_taskbars[index - 1]._class, tbs[index]._class);
                assert_eq!(
                    tbs._secondary_taskbars[index - 1]._window_handle,
                    tbs[index]._window_handle
                );
            }
        }
    }
}

#[should_panic]
#[test]
fn taskbars_index_out_of_bounds() {
    let tbs = find_taskbars().unwrap();
    _ = tbs[tbs.count()];
}
