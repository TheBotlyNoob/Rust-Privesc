use std::ptr;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

/// Returns true if the current process has admin rights, otherwise false.
pub fn is_elevated() -> bool {
    let mut handle = HANDLE::default();

    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle).as_bool() } {
        let mut elevation = TOKEN_ELEVATION::default();
        let mut ret_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let elevated = if unsafe {
            GetTokenInformation(
                handle,
                TokenElevation,
                ptr::addr_of_mut!(elevation) as _,
                ret_size,
                &mut ret_size,
            )
            .as_bool()
        } {
            elevation.TokenIsElevated != 0
        } else {
            false
        };

        unsafe { CloseHandle(handle) };

        elevated
    } else {
        false
    }
}
