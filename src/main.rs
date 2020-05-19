#[cfg(windows)] extern crate winapi;
use std::io::Error;

fn print_message_gui(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
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

    print_message_gui("yeah").unwrap();
}
