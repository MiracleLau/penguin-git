use serde::{Deserialize, Serialize};
use std::fs;
use crate::config;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub git_impl: Option<String>,     // "cli" | "gix"
    pub git_path: Option<String>,
    pub git_name: Option<String>,
    pub git_email: Option<String>,
}

pub fn load() -> AppSettings {
    let path = config::settings_path();
    if !path.exists() {
        return AppSettings::default();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(settings: &AppSettings) -> Result<(), String> {
    let path = config::settings_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
