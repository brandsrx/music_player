use std::process::Command;

pub fn get_system_volume_linux()->Option<u8>{
    let output = Command::new("pactl")
        .args(&["get-sink-volume","@DEFAULT_SINK@"])
        .output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(percent) = line.split("/").nth(1) {
            return percent.trim().trim_end_matches('%').parse::<u8>().ok();
        }
    }
    None 
}