#[tauri::command]
pub fn check_git_installed() -> bool {
    which::which("git").is_ok()
}

#[tauri::command]
pub fn get_git_version() -> Option<String> {
    if !check_git_installed() {
        return None;
    }
    let output = std::process::Command::new("git")
        .arg("--version")
        .output()
        .ok()?;
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Some(version)
}
