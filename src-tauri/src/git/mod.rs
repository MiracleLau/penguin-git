pub mod models;
pub mod backend;
pub mod cli_backend;
mod gix_backend;

use backend::GitBackend;
use crate::settings;

pub fn create_backend() -> Box<dyn GitBackend + Send> {
    let s = settings::load();
    match s.git_impl.as_deref() {
        Some("cli") => {
            Box::new(cli_backend::CliBackend::new(s.git_path))
        }
        _ => {
            Box::new(gix_backend::GixBackend)
        }
    }
}

// Re-export types for commands
pub use models::{FileStatus, CommitInfo};
