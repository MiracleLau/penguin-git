use crate::git;
use crate::git::backend::GitBackend;
use crate::settings;
use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use serde_json;
use std::path::Path;
use std::sync::Mutex;
use tauri::Emitter;

pub struct WatcherHandle(
    pub  Mutex<
        Option<
            notify_debouncer_full::Debouncer<
                notify::RecommendedWatcher,
                notify_debouncer_full::FileIdMap,
            >,
        >,
    >,
);

fn backend() -> Box<dyn GitBackend + Send> {
    git::create_backend()
}

fn identity() -> (String, String) {
    let s = settings::load();
    let name = s.git_name.unwrap_or_else(|| "Penguin Git".to_string());
    let email = s.git_email.unwrap_or_else(|| "penguin@local".to_string());
    (name, email)
}

#[tauri::command]
pub fn get_status(path: String) -> Result<Vec<git::FileStatus>, String> {
    backend().get_status(&path)
}

#[tauri::command]
pub fn get_diff(path: String, file: String) -> Result<String, String> {
    backend().get_diff(&path, &file)
}

#[tauri::command]
pub fn stage_files(path: String, files: Vec<String>) -> Result<(), String> {
    backend().stage_files(&path, &files)
}

#[tauri::command]
pub fn stage_all(path: String) -> Result<(), String> {
    backend().stage_all(&path)
}

#[tauri::command]
pub fn unstage_files(path: String, files: Vec<String>) -> Result<(), String> {
    backend().unstage_files(&path, &files)
}

#[tauri::command]
pub fn commit(path: String, message: String) -> Result<git::CommitInfo, String> {
    let (name, email) = identity();
    backend().commit_with_identity(&path, &message, &name, &email)
}

#[tauri::command]
pub fn get_log(path: String, limit: u32) -> Result<Vec<git::CommitInfo>, String> {
    backend().get_log(&path, limit)
}

#[tauri::command]
pub fn start_watching(
    path: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, WatcherHandle>,
) -> Result<(), String> {
    let mut handle = state.0.lock().map_err(|e| e.to_string())?;
    *handle = None;

    let app_clone = app.clone();
    let mut debouncer = new_debouncer(
        std::time::Duration::from_millis(500),
        None,
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                for event in &events {
                    for path in &event.paths {
                        let path_str = path.to_string_lossy();
                        if path_str.contains("/.git/")
                            || path_str.contains("\\.git\\")
                            || path_str.contains("/node_modules/")
                            || path_str.contains("\\node_modules\\")
                            || path_str.contains("/target/")
                            || path_str.contains("\\target\\")
                        {
                            continue;
                        }
                        let _ = app_clone.emit(
                            "file-changed",
                            serde_json::json!({
                                "path": path_str
                            }),
                        );
                    }
                }
            }
        },
    )
    .map_err(|e| format!("创建文件监听失败: {}", e))?;

    let watch_path = Path::new(&path).canonicalize().map_err(|e| e.to_string())?;
    debouncer
        .watch(&watch_path, RecursiveMode::Recursive)
        .map_err(|e| format!("监听目录失败: {}", e))?;

    *handle = Some(debouncer);
    Ok(())
}

#[tauri::command]
pub fn stop_watching(state: tauri::State<'_, WatcherHandle>) -> Result<(), String> {
    let mut handle = state.0.lock().map_err(|e| e.to_string())?;
    *handle = None;
    Ok(())
}

#[tauri::command]
pub fn get_ahead_behind(path: String) -> Result<(usize, usize), String> {
    backend().get_ahead_behind(&path)
}

#[tauri::command]
pub fn reset_to_commit(path: String, hash: String) -> Result<(), String> {
    backend().reset_to_commit(&path, &hash)
}

#[tauri::command]
pub fn revert_to_commit(path: String, hash: String, message: String) -> Result<(), String> {
    let (name, email) = identity();
    backend().revert_to_commit(&path, &hash, &message, &name, &email)
}

#[tauri::command]
pub fn discard_changes(path: String) -> Result<(), String> {
    backend().discard_changes(&path)
}
