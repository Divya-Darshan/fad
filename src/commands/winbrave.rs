use std::process::Command;

pub fn run() {
    // Check if Brave is installed by searching for its ID.
    // Exit code 0 means found (installed), non-zero means not found.
    let is_installed = Command::new("winget")
        .args(["list", "--id", "Brave.Brave", "-e"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if is_installed {
        println!("Brave is already installed.");
    } else {
        println!("Brave not found. Installing...");
        let _ = Command::new("winget")
            .args([
                "install",
                "--id",
                "Brave.Brave",
                "-e",
                "--silent",
                "--accept-package-agreements",
                "--accept-source-agreements",
            ])
            .status();
    }
}   