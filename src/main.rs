use std::io::Error;
use std::ffi::OsString;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;
// use winapi::um::winuser::WNDENUMPROC;
use winapi::shared::minwindef::BOOL;
use std::ptr::null_mut;
use winapi::um::winuser::{EnumDesktopWindows, GetWindowTextW, IsWindowVisible};
use std::slice;
use std::os::windows::prelude::*;

unsafe extern "system" fn callback(handle: HWND, _params: LPARAM) -> BOOL {
    let name = Vec::with_capacity(1024); 
    let ptr = name.as_ptr();
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtexta
    let return_value = GetWindowTextW(handle, ptr as *mut u16, 1024);
    // https://docs.microsoft.com/ko-kr/windows/win32/api/winuser/nf-winuser-iswindowvisible?redirectedfrom=MSDN
    if return_value > 0 && IsWindowVisible(handle) != 0 {
        // filter windows through has window's title bar and is visible
        let slice = slice::from_raw_parts(ptr, return_value as usize);
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
        EnumDesktopWindows(null_mut(), Some(callback), 0 as LPARAM)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

fn main() {
    // if OS is not Windows then terminate the program.
    if cfg!(not(windows)) {
        eprintln!("You are not a Windows user!");
        std::process::exit(0x01);
    }

    // unsafe { show_currnet_window_info(); }
    listing_windows().unwrap();
}
