use std::result::Result;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

/// Gets a region handle that contains details of a window
pub fn get_region(window_handle: HWND) -> Result<HRGN, String> {
    let region_handle = create_rect_region(0, 0, 0, 0).unwrap();
    unsafe {
        match GetWindowRgn(window_handle, region_handle) as u32 {
            NULLREGION | SIMPLEREGION | COMPLEXREGION => return Ok(region_handle),
            ERROR => return Err("Failed to get window region".to_string()),
            _ => return Err("Unknown response".to_string()),
        }
    }
}

/// Creates a region handle that refers to a region of the specified dimensions
pub fn create_rect_region(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<HRGN, String> {
    unsafe {
        let empty_region = CreateRectRgn(x1, y1, x2, y2);
        if empty_region.is_invalid() {
            return Err("Failed to create rectangular region".to_string());
        }
        return Ok(empty_region);
    }
}

/// Tell Windows to clean up the region object pointed to by a region handle
pub fn free_region(region: HRGN) -> Result<(), String> {
    unsafe {
        // necessary, cant cast in match
        match DeleteObject(region) {
            BOOL(0) => return Err("Failed to delete region".to_string()),
            _ => return Ok(()),
        }
    }
}
