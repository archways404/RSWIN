extern crate winapi;

use winapi::um::winuser::{MessageBoxW, MB_OK};
use std::ptr::null_mut;

fn main() {
    let title = "Hello, World!";
    let text = "This is a message box from Rust!";

    // Convert Rust strings to wide strings (UTF-16)
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_text: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        MessageBoxW(null_mut(), wide_text.as_ptr(), wide_title.as_ptr(), MB_OK);
    }
}

