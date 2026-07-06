use super::models::{CommitInfo, FileStatus};

pub trait GitBackend: Send {
    fn get_status(&self, path: &str) -> Result<Vec<FileStatus>, String>;
    fn get_diff(&self, path: &str, file: &str) -> Result<String, String>;
    fn stage_all(&self, path: &str) -> Result<(), String>;
    fn stage_files(&self, path: &str, files: &[String]) -> Result<(), String>;
    fn unstage_files(&self, path: &str, files: &[String]) -> Result<(), String>;
    fn commit_with_identity(
        &self,
        path: &str,
        message: &str,
        name: &str,
        email: &str,
    ) -> Result<CommitInfo, String>;
    fn get_log(&self, path: &str, limit: u32) -> Result<Vec<CommitInfo>, String>;
    fn get_ahead_behind(&self, path: &str) -> Result<(usize, usize), String>;
    fn reset_to_commit(&self, path: &str, hash: &str) -> Result<(), String>;
    fn revert_to_commit(
        &self,
        path: &str,
        hash: &str,
        message: &str,
        name: &str,
        email: &str,
    ) -> Result<(), String>;
    fn discard_changes(&self, path: &str) -> Result<(), String>;
}
