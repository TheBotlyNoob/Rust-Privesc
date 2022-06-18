fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    windir_with_silentcleanup_task::elevate(|| {
        let _ = std::process::Command::new("cmd.exe").spawn();
    })
}
