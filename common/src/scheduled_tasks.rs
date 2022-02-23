use std::{ffi::CString, path::Path, ptr};
use windows::Win32::{
  Foundation::{BSTR, PWSTR},
  System::{
    Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
    Ole::VariantClear,
    TaskScheduler::{ITaskService, TaskScheduler, TASK_RUN_IGNORE_CONSTRAINTS},
  },
  UI::Shell::PropertiesSystem::InitVariantFromStringArray,
};

pub fn run_task(
  task: impl AsRef<Path>,
  params: impl Into<Option<Vec<String>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let task = task.as_ref();

  println!("[+] Running the {} task...", task.display());

  unsafe { CoInitialize(ptr::null_mut())? };

  unsafe {
    let task_service =
      CoCreateInstance::<_, ITaskService>(&TaskScheduler, None, CLSCTX_INPROC_SERVER).unwrap();

    task_service.Connect(None, None, None, None)?;

    let task = task_service
      .GetFolder(BSTR::from(task.parent().unwrap().to_str().unwrap()))?
      .GetTask(BSTR::from(task.file_name().unwrap().to_str().unwrap()))?;

    let variant = if let Some(params) = params.into() {
      let mut params = params
        .into_iter()
        .map(|param| CString::new(param).unwrap())
        .collect::<Vec<CString>>();

      let pwstr = PWSTR(params.as_mut_ptr() as _);
      Some(InitVariantFromStringArray(
        ptr::addr_of!(pwstr),
        params.len() as _,
      )?)
    } else {
      None
    };

    task
      .RunEx(variant.clone(), TASK_RUN_IGNORE_CONSTRAINTS, 0, None)
      .unwrap();

    if let Some(mut variant) = variant {
      VariantClear(ptr::addr_of_mut!(variant))?;
    }
  }

  unsafe { CoUninitialize() };

  Ok(())
}
