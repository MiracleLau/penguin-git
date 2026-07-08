use crate::settings;
use std::process::Command;

fn git_path() -> String {
    crate::git::resolve_git_path()
}

fn git_global_config(args: &[&str]) -> Result<String, String> {
    let output = Command::new(git_path())
        .args(args)
        .output()
        .map_err(|e| format!("执行 git 命令失败: {}", e))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if err.is_empty() {
            "git 命令执行失败".into()
        } else {
            err
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub fn get_settings() -> settings::AppSettings {
    settings::load()
}

#[tauri::command]
pub fn set_settings(s: settings::AppSettings) -> Result<(), String> {
    settings::save(&s)
}

#[tauri::command]
pub fn get_git_global_config(key: String) -> Option<String> {
    git_global_config(&["config", "--global", &key])
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tauri::command]
pub fn set_git_global_config(key: String, value: String) -> Result<(), String> {
    git_global_config(&["config", "--global", &key, &value]).map(|_| ())
}

#[tauri::command]
pub fn detect_git_path() -> Option<String> {
    which::which("git")
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}
