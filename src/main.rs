#[cfg(windows)] extern crate winapi;
use std::io::Error;

extern "C" fn callback(handle: winapi::shared::basetsd::HANDLE_PTR, params: winapi::shared::minwindef::LPARAM) {
    println!("I'm called from C with value {0}", handle);
}

fn listing_windows() -> Result<i32, Error> {
    // use std::ffi::OsStr;
    // use std::iter::once;
    // use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::EnumDesktopWindows;
    // let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        EnumDesktopWindows(null_mut(), callback, 0)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

/*
https://docs.microsoft.com/ko-kr/windows/win32/api/winuser/nf-winuser-enumdesktopwindows?redirectedfrom=MSDN 써서 현재 데스크톱에서 실행중인 프로그램 목록 얻어오고
Rust 에서 NULL을 넘기는법
RUST 에서 함수 포인터를 넘기는법 몰겠네
*/

fn main() {
    // if OS is not Windows then terminate the program.
    if cfg!(not(windows)) {
        eprintln!("You are not a Windows user!");
        std::process::exit(0x01);
    }

    listing_windows().unwrap();
}
