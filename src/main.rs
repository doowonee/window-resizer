use std::io::Error;
use std::ffi::OsString;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;
// use winapi::um::winuser::WNDENUMPROC;
use winapi::shared::minwindef::BOOL;
use std::ptr::null_mut;
use winapi::um::winuser::{EnumDesktopWindows, GetWindowTextW, IsWindowVisible, GetTopWindow, GetWindow, GW_HWNDNEXT};
use std::slice;
use std::os::windows::prelude::*;

const BUFF_SIZE: usize = 1024;

unsafe extern "system" fn cb_window_found(handle: HWND, _params: LPARAM) -> BOOL {
    let name = Vec::with_capacity(BUFF_SIZE); 
    let ptr = name.as_ptr();
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw
    let length = GetWindowTextW(handle, ptr as *mut u16, BUFF_SIZE as i32);
    // https://docs.microsoft.com/ko-kr/windows/win32/api/winuser/nf-winuser-iswindowvisible?redirectedfrom=MSDN
    if length > 0 && IsWindowVisible(handle) != 0 {
        // filter windows through has window's title bar and is visible
        let slice = slice::from_raw_parts(ptr, length as usize);
        println!("#@ callback from EnumDesktopWindows: {:?}-{:?}", handle, OsString::from_wide(slice));
    }
    1 as BOOL
}

fn listing_windows() -> Result<i32, Error> {
    // use std::ffi::OsStr;
    // use std::iter::once;
    // use std::os::windows::ffi::OsStrExt;
    // let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopwindows
        EnumDesktopWindows(null_mut(), Some(cb_window_found), 0 as LPARAM)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

unsafe fn walk_windows() {
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow
    let mut window = GetTopWindow(null_mut());

    while !window.is_null() {
        let name = Vec::with_capacity(BUFF_SIZE); 
        let ptr = name.as_ptr();
        let length = GetWindowTextW(window, ptr as *mut u16, BUFF_SIZE as i32);
        if length > 0 && IsWindowVisible(window) != 0 {
            let slice = slice::from_raw_parts(ptr, length as usize);
            println!("#@ window title: {:?}", OsString::from_wide(slice));
        }
        // https://docs.microsoft.com/ko-kr/windows/win32/api/winuser/nf-winuser-getwindow
        window = GetWindow(window, GW_HWNDNEXT);
    }
}

fn main() {
    // if OS is not Windows then terminate the program.
    if cfg!(not(windows)) {
        eprintln!("You are not a Windows user!");
        std::process::exit(0x01);
    }

    listing_windows().unwrap();
    println!("------- #@ walk 방식 -------");
    unsafe { walk_windows(); }
}
