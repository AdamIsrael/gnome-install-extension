/// Simple interface to GNOME
use std::process::Command;

use regex::Regex;

/// Enable GNOME extension
pub fn enable_extension(uuid: &str) {
    // gnome-extensions enable $uuid
    if cfg!(target_os = "linux") {
        Command::new("gnome-extension")
            .args(["enable", uuid])
            .output()
            .expect("Can't find gnome-extension in path")
    } else {
        panic!("Only Linux is supported.")
    };
}

/// Get the GNOME Shell version.
pub fn get_shell_version() -> Result<i32, Box<dyn std::error::Error>> {
    // TODO: what if `gnome-shell` isn't in our path (like inside a container)?
    let output = if cfg!(target_os = "linux") {
        Command::new("gnome-shell")
            .args(["--version"])
            .output()
            .expect("Can't find gnome-shell in path")
    } else {
        panic!("Only Linux is supported.")
    };

    // Parse the output
    let tmp = String::from_utf8(output.stdout).unwrap();
    let re = Regex::new(r"^GNOME Shell (\d+).\d+\n$").unwrap();
    let caps = re
        .captures(tmp.as_str())
        .ok_or("Couldn't parse GNOME Shell version")?;
    let m = caps.get(1).unwrap().as_str();

    // Parse out the number.
    let version = m.parse::<i32>().unwrap_or(0);

    Ok(version)
}
