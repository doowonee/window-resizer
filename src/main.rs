use std::io::Error;
use std::ffi::CStr;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;
// use winapi::um::winuser::WNDENUMPROC;
use winapi::shared::minwindef::BOOL;
use std::ptr::null_mut;
use winapi::um::winuser::{EnumDesktopWindows, GetWindowTextA};

unsafe extern "system" fn callback(handle: HWND, _params: LPARAM) -> BOOL {
    let name = Vec::with_capacity(1024); 
    let ptr = name.as_ptr();
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtexta
    let return_value = GetWindowTextA(handle, ptr as *mut i8, 1024);
    println!("#@ callback from EnumDesktopWindows: {:?}-{:?}:{:?}", handle, CStr::from_ptr(ptr), return_value);
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

    listing_windows().unwrap();
}
