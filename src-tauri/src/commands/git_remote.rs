use std::process::Command;

fn git_path() -> String {
    crate::git::resolve_git_path()
}

fn git(args: &[&str], cwd: &str) -> Result<String, String> {
    let output = Command::new(git_path())
        .args(args)
        .current_dir(cwd)
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
pub fn add_remote(path: String, name: String, url: String) -> Result<(), String> {
    let existing = git(&["remote", "get-url", &name], &path);
    match existing {
        Ok(_) => {
            let _ = git(&["remote", "set-url", &name, &url], &path)?;
        }
        Err(_) => {
            let _ = git(&["remote", "add", &name, &url], &path)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn remove_remote(path: String, name: String) -> Result<(), String> {
    git(&["remote", "remove", &name], &path).map(|_| ())
}

#[tauri::command]
pub fn push(path: String) -> Result<String, String> {
    git(&["push", "--progress"], &path)
}

#[tauri::command]
pub fn pull(path: String) -> Result<String, String> {
    git(&["pull", "--progress"], &path)
}

#[tauri::command]
pub fn fetch(path: String) -> Result<(), String> {
    git(&["fetch", "--prune"], &path).map(|_| ())
}

#[tauri::command]
pub fn get_remote_url(path: String) -> Option<String> {
    git(&["remote", "get-url", "origin"], &path)
        .ok()
        .map(|s| s.trim().to_string())
}
