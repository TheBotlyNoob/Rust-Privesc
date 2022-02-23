use std::{ffi::CString, path::Path};
use windows::Win32::{
  Foundation::PSTR,
  System::Registry::{
    RegCloseKey, RegDeleteValueA, RegOpenKeyExA, RegSetValueExA, HKEY, KEY_WRITE, REG_SZ,
  },
};

pub use windows::Win32::System::Registry::{
  HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for StringError {}

pub fn set_value(
  root: HKEY,
  value_path: impl AsRef<Path>,
  value: impl AsRef<str>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let value = value.as_ref();
  let value_path = value_path.as_ref();

  println!("[+] Setting the {} registry value...", value_path.display());

  let mut hkey = HKEY::default();

  let subkey = CString::new(
    value_path
      .parent()
      .unwrap()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap(),
  )?;
  let subkey = PSTR(subkey.as_ptr() as _);

  let value_name = CString::new(value_path.file_name().unwrap().to_str().unwrap())?;
  let value_name = PSTR(value_name.as_ptr() as _);

  {
    let res = unsafe { RegOpenKeyExA(root, subkey, 0, KEY_WRITE, &mut hkey) };

    if res != 0 {
      return Err(Box::new(StringError(format!(
        "Error calling RegOpenKeyExA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      ))));
    };
  }

  {
    let res = unsafe {
      RegSetValueExA(
        hkey,
        value_name,
        0,
        REG_SZ,
        value.as_ptr(),
        value.len() as u32,
      )
    };

    if res != 0 {
      unsafe { RegCloseKey(hkey) };

      return Err(Box::new(StringError(format!(
        "Error calling RegSetValueExA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32),
      ))));
    };

    unsafe { RegCloseKey(hkey) };
  }

  Ok(())
}

pub fn delete_value<T: AsRef<Path>>(
  root: HKEY,
  value_path: &T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let value_path = value_path.as_ref();

  println!("[+] Deleting the {} registry key...", value_path.display());

  let mut hkey = HKEY::default();

  let subkey = CString::new(
    value_path
      .parent()
      .unwrap()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap(),
  )
  .unwrap();
  let subkey = PSTR(subkey.as_ptr() as _);

  let value_name = CString::new(value_path.file_name().unwrap().to_str().unwrap())?;
  let value_name = PSTR(value_name.as_ptr() as _);

  {
    let res = unsafe { RegOpenKeyExA(root, subkey, 0, KEY_WRITE, &mut hkey) };

    if res != 0 {
      return Err(Box::new(StringError(format!(
        "Error calling RegOpenKeyExA: {:#?}",
        std::io::Error::from_raw_os_error(res as i32)
      ))));
    };
  }

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

  Ok(())
}
