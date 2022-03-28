# POCs built in Rust

## UAC Bypasses

- [mock-trusted-directories](./uac-bypasses/mock-trusted-directories) - bypasses UAC by using a fake trusted directory.
- [windir-with-silentcleanup-task](./uac-bypasses/windir-with-silentcleanup-task) - bypasses UAC by mocking the `WinDir` environment variable, and using the `SilentCleanup` task.

All of these POCs execute `cmd.exe` by default.
