#![feature(restricted_std, exit_status_error)]

use std::{io::Write, process::{Command, Output, Stdio}, fs::File};

#[allow(clippy::upper_case_acronyms, unused)]
enum BootType {
    UEFI,
    BIOS
}

const BOOT_TYPE: BootType = BootType::UEFI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir   = env!("OUT_DIR");
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd
        // Freeze QEMU instead of rebooting
        // .args([ "-action", "reboot=shutdown,shutdown=pause" ])
        // Send serial output to stdout
        // .args([ "-serial", "stdio" ])
        // Display options
        // .args([ "-display", "gtk,gl=on,full-screen=on" ])
        // Increase memory available to QEMU
        .args([ "-m", "4G" ]);


    match BOOT_TYPE {
        BootType::UEFI => {
            cmd.arg("-bios")
                .arg(ovmf_prebuilt::ovmf_pure_efi());
            cmd.arg("-drive")
                .arg(format!("format=raw,file={uefi_path}"));

            println!("UEFI img located at: {uefi_path}");
        },
        BootType::BIOS => {
            cmd.arg("-drive")
                .arg(format!("format=raw,file={bios_path}"));

            println!("BIOS img located at: {bios_path}");
        },
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let wait = child.wait().unwrap();
    wait.exit_ok().unwrap();

    Ok(())
}

