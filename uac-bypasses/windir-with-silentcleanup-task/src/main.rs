fn main() {
  windir_with_silentcleanup_task::elevate(|| {
    std::process::Command::new("cmd.exe").spawn().unwrap();
  });
}
