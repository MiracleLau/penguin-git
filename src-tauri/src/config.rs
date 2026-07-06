use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("penguin-git")
}

pub fn recent_projects_path() -> PathBuf {
    config_dir().join("recent.json")
}

pub fn settings_path() -> PathBuf {
    config_dir().join("settings.json")
}

pub fn master_key_path() -> PathBuf {
    config_dir().join("master.key")
}

pub fn credentials_path() -> PathBuf {
    config_dir().join("credentials.enc")
}
