use std::{env, process::Command};

fn main() {
  println!("cargo:rerun-if-changed=dll/");

  if !Command::new("cargo")
    .args([
      "rustc",
      "--release",
      "--lib",
      "--manifest-path=dll/Cargo.toml",
      "--target=x86_64-pc-windows-gnu",
      "--",
      "-o",
      &format!("{}/winmm", env::var("OUT_DIR").unwrap()),
    ])
    .spawn()
    .unwrap()
    .wait()
    .unwrap()
    .success()
  {
    panic!("Failed to compile the dll, please check the output above");
  }
}
