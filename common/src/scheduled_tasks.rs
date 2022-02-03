use std::{path::Path, ptr};
use windows::Win32::{
  Foundation::BSTR,
  System::{
    Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
    TaskScheduler::{ITaskService, TaskScheduler, TASK_RUN_IGNORE_CONSTRAINTS},
  },
};

pub fn run_task<T: AsRef<Path>>(task: &T) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let task = task.as_ref();

  println!("[+] Running the {} task...", task.display());

  unsafe { CoInitialize(ptr::null_mut())? };

  unsafe {
    let task_service =
      CoCreateInstance::<_, ITaskService>(&TaskScheduler, None, CLSCTX_INPROC_SERVER).unwrap();

    task_service.Connect(None, None, None, None)?;

    task_service
      .GetFolder(BSTR::from(task.parent().unwrap().to_str().unwrap()))?
      .GetTask(BSTR::from(task.file_name().unwrap().to_str().unwrap()))?
      .RunEx(None, TASK_RUN_IGNORE_CONSTRAINTS, 0, None)?
  };

  unsafe { CoUninitialize() };

  Ok(())
}
