use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config;

#[tauri::command]
pub fn is_git_repo(path: String) -> bool {
    Path::new(&path).join(".git").exists()
}

#[tauri::command]
pub fn init_repo(path: String, gitignore_content: Option<String>) -> Result<(), String> {
    gix::init(&path).map_err(|e| format!("初始化仓库失败: {}", e))?;
    if let Some(content) = gitignore_content {
        if !content.trim().is_empty() {
            std::fs::write(Path::new(&path).join(".gitignore"), content)
                .map_err(|e| format!("写入 .gitignore 失败: {}", e))?;
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub is_git_repo: bool,
    pub has_remote: bool,
    pub remote_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentProject {
    pub path: String,
    pub name: String,
    pub last_opened: i64,
}

#[tauri::command]
pub fn open_project(path: String) -> Result<ProjectInfo, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err("路径不存在".into());
    }
    let name = p.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let is_git = p.join(".git").exists();
    let (has_remote, remote_url) = if is_git {
        match gix::open(&path) {
            Ok(repo) => {
                let url = repo.find_remote("origin").ok()
                    .and_then(|r| r.url(gix::remote::Direction::Fetch).map(|u| u.to_string()));
                (url.is_some(), url)
            }
            Err(_) => (false, None),
        }
    } else {
        (false, None)
    };

    // Save to recent
    save_recent_project_inner(&path, &name);

    Ok(ProjectInfo { path, name, is_git_repo: is_git, has_remote, remote_url })
}

fn save_recent_project_inner(path: &str, name: &str) {
    let mut projects = get_recent_projects_inner();
    projects.retain(|p| p.path != path);
    projects.insert(0, RecentProject {
        path: path.to_string(),
        name: name.to_string(),
        last_opened: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
    });
    projects.truncate(20);
    save_recent_list(&projects);
}

fn get_recent_projects_inner() -> Vec<RecentProject> {
    let path = config::recent_projects_path();
    if !path.exists() {
        return Vec::new();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_recent_list(projects: &[RecentProject]) {
    let path = config::recent_projects_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    if let Ok(json) = serde_json::to_string_pretty(projects) {
        std::fs::write(&path, json).ok();
    }
}

#[tauri::command]
pub fn get_recent_projects() -> Vec<RecentProject> {
    get_recent_projects_inner()
}

#[tauri::command]
pub fn save_recent_project(path: String, name: String) {
    save_recent_project_inner(&path, &name);
}
