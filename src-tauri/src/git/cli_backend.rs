use std::process::Command;
use super::models::{CommitInfo, FileStatus};
use super::backend::GitBackend;

pub struct CliBackend {
    git_path: String,
}

impl CliBackend {
    pub fn new(git_path: Option<String>) -> Self {
        let path = git_path.unwrap_or_else(|| "git".to_string());
        Self { git_path: path }
    }

    fn git(&self, args: &[&str], cwd: &str) -> Result<String, String> {
        let output = Command::new(&self.git_path)
            .args(args)
            .current_dir(cwd)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {}", e))?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(if err.is_empty() { "git 命令执行失败".into() } else { err });
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl GitBackend for CliBackend {
    fn get_status(&self, path: &str) -> Result<Vec<FileStatus>, String> {
        let output = self.git(&["status", "--porcelain"], path)?;
        let mut files = Vec::new();
        for line in output.lines() {
            if line.is_empty() { continue; }
            if line.as_bytes().len() < 3 { continue; }

            let xy = &line[..2];
            let file = &line[3..];

            let staged = xy.as_bytes()[0] != b' ';

            let status = match (xy.as_bytes()[0], xy.as_bytes()[1]) {
                (b'?', _) | (_, b'?') => "new",
                (b'M', _) | (_, b'M') => "modified",
                (b'D', _) | (_, b'D') => "deleted",
                (b'R', _) | (_, b'R') => "renamed",
                (b'A', _) | (_, b'A') => "new",
                _ => "modified",
            };

            let display_path = if let Some(idx) = file.find(" -> ") {
                &file[idx + 4..]
            } else {
                file
            };

            files.push(FileStatus {
                path: display_path.trim().to_string(),
                status: status.to_string(),
                staged,
                additions: 0,
                deletions: 0,
            });
        }
        Ok(files)
    }

    fn get_diff(&self, path: &str, file: &str) -> Result<String, String> {
        self.git(&["diff", "--", file], path)
    }

    fn stage_all(&self, path: &str) -> Result<(), String> {
        self.git(&["add", "-A"], path).map(|_| ())
    }

    fn stage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        if files.is_empty() { return Ok(()); }
        let mut args = vec!["add", "--"];
        for f in files { args.push(f); }
        self.git(&args, path).map(|_| ())
    }

    fn unstage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        if files.is_empty() { return Ok(()); }
        let mut args = vec!["reset", "HEAD", "--"];
        for f in files { args.push(f); }
        self.git(&args, path).map(|_| ())
    }

    fn commit_with_identity(&self, path: &str, message: &str, name: &str, email: &str) -> Result<CommitInfo, String> {
        self.stage_all(path)?;

        let status = self.get_status(path)?;
        if status.is_empty() {
            return Err("没有变更可保存".into());
        }

        let output = self.git(&[
            "-c", &format!("user.name={}", name),
            "-c", &format!("user.email={}", email),
            "commit", "-m", message
        ], path)?;

        let hash = output.lines().find(|l| l.contains(']'))
            .and_then(|l| {
                l.split(']').next()?
                    .split_whitespace().last()
                    .map(|s| s.to_string())
            })
            .unwrap_or_default();

        Ok(CommitInfo {
            short_hash: hash.chars().take(7).collect(),
            hash,
            message: message.to_string(),
            author: name.to_string(),
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        })
    }

    fn get_log(&self, path: &str, limit: u32) -> Result<Vec<CommitInfo>, String> {
        let output = self.git(&[
            "log", "--format=%H|%an|%at|%s",
            &format!("--max-count={}", limit),
        ], path)?;

        let mut commits = Vec::new();
        for line in output.lines() {
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.splitn(4, '|').collect();
            if parts.len() < 4 { continue; }
            let hash = parts[0].to_string();
            commits.push(CommitInfo {
                short_hash: hash.chars().take(7).collect(),
                hash,
                author: parts[1].to_string(),
                time: parts[2].parse().unwrap_or(0),
                message: parts[3].to_string(),
            });
        }
        Ok(commits)
    }

    fn get_ahead_behind(&self, path: &str) -> Result<(usize, usize), String> {
        let branch = self.git(&["rev-parse", "--abbrev-ref", "HEAD"], path)?;
        let branch = branch.trim();
        if branch == "HEAD" { return Ok((0, 0)); }

        let tracking = self.git(&[
            "rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"
        ], path);
        let tracking = match tracking {
            Ok(t) => t.trim().to_string(),
            Err(_) => return Ok((0, 0)),
        };

        let ahead = self.git(&[
            "rev-list", "--count", &format!("{}..{}", tracking, branch)
        ], path)?;

        let behind = self.git(&[
            "rev-list", "--count", &format!("{}..{}", branch, tracking)
        ], path)?;

        Ok((
            ahead.trim().parse().unwrap_or(0),
            behind.trim().parse().unwrap_or(0),
        ))
    }

    fn reset_to_commit(&self, path: &str, hash: &str) -> Result<(), String> {
        self.git(&["reset", "--hard", hash], path)?;
        self.git(&["clean", "-fd"], path)?;
        let head = self.git(&["rev-parse", "HEAD"], path)?;
        let target = self.git(&["rev-parse", hash], path)?;
        if head.trim() != target.trim() {
            return Err(format!("回退校验失败: HEAD={} 目标={}", head.trim(), target.trim()));
        }
        Ok(())
    }

    fn revert_to_commit(&self, path: &str, hash: &str, message: &str, name: &str, email: &str) -> Result<(), String> {
        self.git(&["read-tree", "--reset", "-u", hash], path)?;
        self.git(&[
            "-c", &format!("user.name={}", name),
            "-c", &format!("user.email={}", email),
            "commit", "-m", message,
        ], path).map(|_| ())
    }

    fn discard_changes(&self, path: &str) -> Result<(), String> {
        self.git(&["reset", "--hard", "HEAD"], path)?;
        self.git(&["clean", "-fd"], path).map(|_| ())
    }
}
