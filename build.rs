use std::{env, process::Command};

fn main() {
  println!("cargo:rerun-if-changed=dll");

  Command::new("cargo")
    .arg("rustc")
    .arg("--lib")
    .arg("--manifest-path=dll/Cargo.toml")
    .output()
    .unwrap();
}
