pub mod backend;
pub mod cli_backend;
mod gix_backend;
pub mod models;

use crate::settings;
use backend::GitBackend;

pub fn create_backend() -> Box<dyn GitBackend + Send> {
    let s = settings::load();
    match s.git_impl.as_deref() {
        Some("cli") => Box::new(cli_backend::CliBackend::new(s.git_path)),
        _ => Box::new(gix_backend::GixBackend),
    }
}

pub fn resolve_git_path() -> String {
    let s = settings::load();
    if let Some(p) = s.git_path {
        return p;
    }
    if let Ok(p) = which::which("git") {
        return p.to_string_lossy().to_string();
    }
    "git".to_string()
}

// Re-export types for commands
pub use models::{CommitInfo, FileStatus};
