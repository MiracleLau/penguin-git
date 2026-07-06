use crate::ssh;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn check_ssh_key() -> ssh::SshKeyInfo {
    tauri::async_runtime::spawn_blocking(|| {
        if let Some(path) = ssh::find_existing_key() {
            let public_key = ssh::read_public_key(&path).ok();
            ssh::SshKeyInfo {
                exists: true,
                public_key,
                path: Some(path.to_string_lossy().to_string()),
            }
        } else {
            ssh::SshKeyInfo {
                exists: false,
                public_key: None,
                path: None,
            }
        }
    })
    .await
    .unwrap_or_else(|_| ssh::SshKeyInfo {
        exists: false,
        public_key: None,
        path: None,
    })
}

#[tauri::command]
pub fn generate_ssh_key(comment: String) -> Result<ssh::SshKeyInfo, String> {
    ssh::generate_ed25519_key(&comment)
}

#[tauri::command]
pub fn get_public_key() -> Result<String, String> {
    let path = ssh::find_existing_key().ok_or_else(|| "未找到 SSH 密钥".to_string())?;
    ssh::read_public_key(&path)
}

#[tauri::command]
pub fn copy_public_key() -> Result<String, String> {
    let path = ssh::find_existing_key().ok_or_else(|| "未找到 SSH 密钥".to_string())?;
    let pub_key = ssh::read_public_key(&path)?;
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(&pub_key).map_err(|e| e.to_string())?;
    Ok(pub_key)
}

#[tauri::command]
pub fn open_platform_ssh_settings(platform: String) -> Result<(), String> {
    let url = match platform.as_str() {
        "github" => "https://github.com/settings/ssh/new",
        "codeberg" => "https://codeberg.org/user/settings/keys",
        _ => return Err(format!("未知平台: {}", platform)),
    };
    open::that(url).map_err(|e| format!("打开链接失败: {}", e))
}

#[tauri::command]
pub async fn test_ssh_connection(host: String) -> TestResult {
    let host_with_prefix = if host.starts_with("git@") {
        host.clone()
    } else {
        format!("git@{}", host)
    };

    let output = tauri::async_runtime::spawn_blocking(move || {
        std::process::Command::new("ssh")
            .arg("-T")
            .arg("-o")
            .arg("StrictHostKeyChecking=accept-new")
            .arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("ConnectTimeout=5")
            .arg("-o")
            .arg("ConnectionAttempts=1")
            .arg(&host_with_prefix)
            .output()
    })
    .await;

    match output {
        Ok(Ok(out)) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stdout = String::from_utf8_lossy(&out.stdout);
            let combined = format!("{}{}", stdout, stderr);

            if combined.contains("successfully authenticated")
                || combined.contains("Welcome")
                || combined.to_lowercase().contains("welcome")
                || out.status.success()
            {
                TestResult {
                    success: true,
                    message: "连接成功！".to_string(),
                }
            } else {
                let msg = combined.trim();
                if msg.is_empty() {
                    TestResult {
                        success: false,
                        message: "连接失败：无响应".to_string(),
                    }
                } else {
                    TestResult {
                        success: false,
                        message: msg.lines().next().unwrap_or("连接失败").to_string(),
                    }
                }
            }
        }
        Ok(Err(e)) => TestResult {
            success: false,
            message: format!("执行 ssh 命令失败: {}", e),
        },
        Err(e) => TestResult {
            success: false,
            message: format!("任务执行失败: {}", e),
        },
    }
}
