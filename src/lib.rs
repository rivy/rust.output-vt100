//! # Output-VT100
//!
//! When you write terminal-based crates, sometimes you might want to use the
//! standard ANSI escaped characters, to display some colors, to display text
//! as bold, italic or whatever. However, you’ve just discovered all your
//! pretty displays that worked like a charm on Linux and Mac look terrible
//! on Windows, because the escaped characters do not work. Rather, they are
//! not activated by default. Then you discover you have to do system calls to
//! Windows directly to activate them in order to get your beautiful text back.
//! What a pain!
//! And this is where this crate comes in action! Simply add it as a dependency
//! for your own crate, and use it like this:
//! ```rust
//! extern crate output_vt100;
//!
//! fn main() {
//!     output_vt100::init();
//!     println!("\x1b[31mThis text is red!\x1b[0m");
//! }
//! ```
//! And that’s it! By calling it once, you have now activated PowerShell’s and
//! CMD’s support for ANSI’s escaped characters on your Windows builds! And
//! you can leave this line in your Unix builds too, it will simply do nothing.

#[cfg(windows)]
pub fn try_init() -> Result<(), ()> {
    // ref: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#EXAMPLE_OF_ENABLING_VIRTUAL_TERMINAL_PROCESSING @@ https://archive.is/L7wRJ#76%

    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    // use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::wincon::{DISABLE_NEWLINE_AUTO_RETURN, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
    use winapi::um::winnt::{FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

    let console_out_name: Vec<u16> = OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
    let mut state: DWORD = 0;
    let mut ret: Result<(), _> = Ok(());
    unsafe {
        // ref: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        // Using `CreateFileW("CONOUT$", ...)` to retrieve the console handle works correctly even if STDOUT and/or STDERR are redirected
        let console_handle = CreateFileW(
            console_out_name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );
        if console_handle == INVALID_HANDLE_VALUE {
            // ret = Err(GetLastError());
            ret = Err(());
        }
        if ret.is_ok() && GetConsoleMode(console_handle, &mut state) == 0 {
            // ret = Err(GetLastError());
            ret = Err(());
        }
        if ret.is_ok() {
            state |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            state &= !DISABLE_NEWLINE_AUTO_RETURN;
            if SetConsoleMode(console_handle, state) == 0 {
                // ret = Err(GetLastError());
                ret = Err(());
            }
        }
    }
    return ret;
}

#[cfg(windows)]
pub fn init() {
    assert_eq!(try_init().is_ok(), true);
}

#[cfg(not(windows))]
pub fn try_init() -> Result<(), ()> {
    Ok(())
}

#[cfg(not(windows))]
pub fn init() {
    ;
}

#[cfg(test)]
mod tests {
    #[test]
    fn activate_vt100() {
        crate::init();
    }
}
