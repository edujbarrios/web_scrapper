use std::process::{Command, Stdio};

// Function to check if a command exists in the PATH
fn command_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .stdout(Stdio::null())
        .status()
        .is_ok()
}

fn main() {
    // Check for Cargo and Rust
    if !command_exists("cargo") || !command_exists("rustc") {
        println!("Rust and Cargo are not detected. Please install Rust and Cargo first.");
        println!("Visit https://rustup.rs/ for instructions.");
        return;
    }

    println!("Rust and Cargo are installed.");

    // Check for geckodriver
    if !command_exists("geckodriver") {
        println!("geckodriver is not installed. It is required for fantoccini.");
        println!("Download it from https://github.com/mozilla/geckodriver/releases");
        println!("And place it in a location in your PATH.");
    } else {
        println!("Found geckodriver.");
    }

    // Check for chromedriver
    if !command_exists("chromedriver") {
        println!("chromedriver is not installed. It is optional but recommended.");
        println!("Download it from https://sites.google.com/a/chromium.org/chromedriver/");
        println!("And place it in a location in your PATH.");
    } else {
        println!("Found chromedriver.");
    }

    // Additional setup tasks can be added here
}
