use super::backend::GitBackend;
use super::models::{CommitInfo, FileStatus};
use std::process::Command;

pub struct GixBackend;

fn cli(args: &[&str], cwd: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("git 执行失败: {}", e))?;
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

impl GitBackend for GixBackend {
    fn get_status(&self, path: &str) -> Result<Vec<FileStatus>, String> {
        let output = cli(&["status", "--porcelain"], path)?;
        let mut files = Vec::new();
        for line in output.lines() {
            if line.is_empty() || line.as_bytes().len() < 3 {
                continue;
            }
            let staged = line.as_bytes()[0] != b' ';
            let file = &line[3..];
            let status = match (line.as_bytes()[0], line.as_bytes()[1]) {
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
        cli(&["diff", "--", file], path)
    }

    fn stage_all(&self, path: &str) -> Result<(), String> {
        cli(&["add", "-A"], path).map(|_| ())
    }

    fn stage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        if files.is_empty() {
            return Ok(());
        }
        let mut args = vec!["add", "--"];
        args.extend(files.iter().map(|s| s.as_str()));
        cli(&args, path).map(|_| ())
    }

    fn unstage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        if files.is_empty() {
            return Ok(());
        }
        let mut args = vec!["reset", "HEAD", "--"];
        args.extend(files.iter().map(|s| s.as_str()));
        cli(&args, path).map(|_| ())
    }

    fn commit_with_identity(
        &self,
        path: &str,
        message: &str,
        name: &str,
        email: &str,
    ) -> Result<CommitInfo, String> {
        self.stage_all(path)?;
        let output = cli(
            &[
                "-c",
                &format!("user.name={}", name),
                "-c",
                &format!("user.email={}", email),
                "commit",
                "-m",
                message,
            ],
            path,
        )?;
        let hash = output
            .lines()
            .find(|l| l.contains(']'))
            .and_then(|l| {
                l.split(']')
                    .next()?
                    .split_whitespace()
                    .last()
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
        let output = cli(
            &[
                "log",
                "--format=%H|%an|%at|%s",
                &format!("--max-count={}", limit),
            ],
            path,
        )?;
        let mut commits = Vec::new();
        for line in output.lines() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(4, '|').collect();
            if parts.len() < 4 {
                continue;
            }
            commits.push(CommitInfo {
                hash: parts[0].to_string(),
                short_hash: parts[0].chars().take(7).collect(),
                author: parts[1].to_string(),
                time: parts[2].parse().unwrap_or(0),
                message: parts[3].to_string(),
            });
        }
        Ok(commits)
    }

    fn get_ahead_behind(&self, path: &str) -> Result<(usize, usize), String> {
        let branch = cli(&["rev-parse", "--abbrev-ref", "HEAD"], path)?;
        let branch = branch.trim();
        if branch == "HEAD" {
            return Ok((0, 0));
        }
        let tracking = match cli(
            &[
                "rev-parse",
                "--abbrev-ref",
                "--symbolic-full-name",
                "@{upstream}",
            ],
            path,
        ) {
            Ok(t) => t.trim().to_string(),
            Err(_) => return Ok((0, 0)),
        };
        let ahead = cli(
            &["rev-list", "--count", &format!("{}..{}", tracking, branch)],
            path,
        )?;
        let behind = cli(
            &["rev-list", "--count", &format!("{}..{}", branch, tracking)],
            path,
        )?;
        Ok((
            ahead.trim().parse().unwrap_or(0),
            behind.trim().parse().unwrap_or(0),
        ))
    }

    fn reset_to_commit(&self, path: &str, hash: &str) -> Result<(), String> {
        cli(&["reset", "--hard", hash], path)?;
        cli(&["clean", "-fd"], path)?;
        let head = cli(&["rev-parse", "HEAD"], path)?;
        let target = cli(&["rev-parse", hash], path)?;
        if head.trim() != target.trim() {
            return Err(format!(
                "回退校验失败: HEAD={} 目标={}",
                head.trim(),
                target.trim()
            ));
        }
        Ok(())
    }

    fn revert_to_commit(
        &self,
        path: &str,
        hash: &str,
        message: &str,
        name: &str,
        email: &str,
    ) -> Result<(), String> {
        cli(&["read-tree", "--reset", "-u", hash], path)?;
        cli(
            &[
                "-c",
                &format!("user.name={}", name),
                "-c",
                &format!("user.email={}", email),
                "commit",
                "-m",
                message,
            ],
            path,
        )
        .map(|_| ())
    }

    fn discard_changes(&self, path: &str) -> Result<(), String> {
        cli(&["reset", "--hard", "HEAD"], path)?;
        cli(&["clean", "-fd"], path).map(|_| ())
    }
}
