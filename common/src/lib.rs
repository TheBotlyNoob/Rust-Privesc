/// Returns true if the current process has admin rights, otherwise false.
pub fn is_elevated() -> bool {
    unsafe { windows::Win32::UI::Shell::IsUserAnAdmin().as_bool() }
}

pub mod scheduled_tasks;

pub mod registry;
