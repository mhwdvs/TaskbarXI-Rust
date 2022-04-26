use windows::Win32::Foundation::*;

pub static mut TASKBARS: Vec<HWND> = Vec::new();
