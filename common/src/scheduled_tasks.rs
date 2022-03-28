use std::{ffi::OsString, os::windows::prelude::OsStrExt, path::Path, ptr};
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::BSTR,
        System::{
            Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
            Ole::VariantClear,
            TaskScheduler::{ITaskService, TaskScheduler, TASK_RUN_IGNORE_CONSTRAINTS},
        },
        UI::Shell::PropertiesSystem::InitVariantFromStringArray,
    },
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
            CoCreateInstance::<_, ITaskService>(&TaskScheduler, None, CLSCTX_INPROC_SERVER)?;

        task_service.Connect(None, None, None, None)?;

        let task = task_service
            .GetFolder(BSTR::from(task.parent().unwrap().to_str().unwrap()))?
            .GetTask(BSTR::from(task.file_name().unwrap().to_str().unwrap()))?;

        let variant = if let Some(params) = params.into() {
            let params = params
                .into_iter()
                .map(|param| {
                    PWSTR(
                        OsString::from(&param)
                            .encode_wide()
                            .collect::<Vec<_>>()
                            .as_mut_ptr(),
                    )
                })
                .collect::<Vec<_>>();

            Some(InitVariantFromStringArray(&params)?)
        } else {
            None
        };

        task.RunEx(variant.clone(), TASK_RUN_IGNORE_CONSTRAINTS.0, 0, None)?;

        if let Some(mut variant) = variant {
            VariantClear(&mut variant)?;
        }
    }

    unsafe { CoUninitialize() };

    Ok(())
}
