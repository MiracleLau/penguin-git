use super::backend::GitBackend;
use super::models::{CommitInfo, FileStatus};
use gix::bstr::BString;
use gix::objs::tree::EntryKind;
use gix::progress;
use gix::status::{self, index_worktree::iter::Summary};
use gix::Repository;
use gix::diff::blob::unified_diff::{ConsumeBinaryHunk, ContextSize};
use gix::diff::blob::{Algorithm, Diff, InternedInput};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct GixBackend;

fn open(path: &str) -> Result<Repository, String> {
    gix::open(path).map_err(|e| format!("打开仓库失败: {}", e))
}

fn workdir(repo: &Repository) -> Result<&Path, String> {
    repo.workdir().ok_or_else(|| "没有工作区".to_string())
}

fn write_file_blob(repo: &Repository, workdir: &Path, rel: &str) -> Result<(EntryKind, gix::hash::ObjectId), String> {
    let full = workdir.join(rel);
    let content = std::fs::read(&full).map_err(|e| format!("读取文件失败 {}: {}", rel, e))?;
    let id = repo.write_blob(&content).map_err(|e| e.to_string())?;
    let meta = std::fs::metadata(&full).map_err(|e| e.to_string())?;
    let kind = if meta.permissions().mode() & 0o111 != 0 {
        EntryKind::BlobExecutable
    } else {
        EntryKind::Blob
    };
    Ok((kind, id.detach()))
}

fn build_tree_from_entries(repo: &Repository, entries: &[(String, EntryKind, gix::hash::ObjectId)]) -> Result<gix::hash::ObjectId, String> {
    let empty = repo.empty_tree();
    let mut editor = repo.edit_tree(empty.id().detach()).map_err(|e| e.to_string())?;
    for (rel, kind, blob_id) in entries {
        editor.upsert(rel.as_str(), *kind, *blob_id).map_err(|e| e.to_string())?;
    }
    Ok(editor.write().map_err(|e| e.to_string())?.detach())
}

fn write_index_from_tree(repo: &Repository, tree_id: &gix::hash::oid) -> Result<(), String> {
    let state = gix::index::State::from_tree(
        tree_id, repo,
        gix::index::validate::path::component::Options::default(),
    ).map_err(|e| e.to_string())?;
    let mut file = gix::index::File::from_state(state, repo.index_path().to_path_buf());
    file.write(gix::index::write::Options::default()).map_err(|e| format!("写入索引失败: {}", e)).map(|_| ())
}

fn walk_worktree(workdir: &Path, root: &Path, files: &mut Vec<String>) -> Result<(), String> {
    if !workdir.is_dir() { return Ok(()); }
    for entry in std::fs::read_dir(workdir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let rel = path.strip_prefix(root).map_err(|_| "路径错误".to_string())?
            .to_string_lossy().replace('\\', "/");
        if rel == ".git" || rel.starts_with(".git/") || rel == "node_modules"
            || rel.starts_with("node_modules/") || rel.starts_with("target/") { continue; }
        if path.is_dir() { walk_worktree(&path, root, files)?; }
        else { files.push(rel); }
    }
    Ok(())
}

fn generate_unified_diff(old: &[u8], new: &[u8]) -> Result<String, String> {
    let input = InternedInput::new(old, new);
    let diff = Diff::compute(Algorithm::Histogram, &input);
    let delegate = ConsumeBinaryHunk::new(String::new(), "\n");
    let unified = gix::diff::blob::UnifiedDiff::new(&diff, &input, delegate, ContextSize::symmetrical(3));
    unified.consume().map_err(|e| format!("生成差异失败: {}", e))
}

fn get_head_blob(repo: &Repository, p: &str) -> Result<Vec<u8>, String> {
    let head_commit = repo.head_commit().map_err(|_| "没有提交记录".to_string())?;
    let tree = head_commit.tree().map_err(|e| e.to_string())?;
    let entry = tree.lookup_entry([p]).map_err(|e| e.to_string())?
        .ok_or_else(|| "文件不在 HEAD 中".to_string())?;
    let data = entry.object().map_err(|e| e.to_string())?;
    Ok(data.data.clone().to_vec())
}

fn is_binary(data: &[u8]) -> bool {
    data[..data.len().min(8000)].contains(&0)
}

fn write_tree_to_worktree(repo: &Repository, tree_id: &gix::hash::oid, workdir: &Path) -> Result<(), String> {
    let tree = repo.find_tree(tree_id).map_err(|e| e.to_string())?;
    for entry in tree.iter().filter_map(Result::ok) {
        let path = entry.filename().to_string();
        let full = workdir.join(&path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        let data = entry.object().map_err(|e| e.to_string())?;
        std::fs::write(&full, data.data.clone()).map_err(|e| format!("写入文件失败 {}: {}", path, e))?;
    }
    Ok(())
}

impl GitBackend for GixBackend {
    fn get_status(&self, path: &str) -> Result<Vec<FileStatus>, String> {
        let repo = open(path)?;
        let mut files = Vec::new();
        let iter = repo.status(progress::Discard)
            .map_err(|e| e.to_string())?
            .into_iter(Vec::<BString>::new())
            .map_err(|e| e.to_string())?;
        for item in iter {
            match item.map_err(|e| format!("状态获取失败: {:?}", e))? {
                status::Item::TreeIndex(change) => {
                    let p = change.location().to_string();
                    let s = match &change {
                        gix::diff::index::Change::Addition { .. } => "new",
                        gix::diff::index::Change::Deletion { .. } => "deleted",
                        gix::diff::index::Change::Modification { .. } => "modified",
                        gix::diff::index::Change::Rewrite { .. } => "renamed",
                    };
                    files.push(FileStatus { path: p, status: s.to_string(), staged: true, additions: 0, deletions: 0 });
                }
                status::Item::IndexWorktree(item) => {
                    let p = item.rela_path().to_string();
                    let (s, skip) = match item.summary() {
                        Some(Summary::Added) => ("new", false),
                        Some(Summary::Modified) => ("modified", false),
                        Some(Summary::Removed) => ("deleted", false),
                        Some(Summary::Renamed) => ("renamed", false),
                        Some(Summary::Copied) => ("new", false),
                        Some(Summary::TypeChange) => ("modified", false),
                        Some(Summary::Conflict) => ("modified", false),
                        Some(Summary::IntentToAdd) => ("new", false),
                        None => ("", true),
                    };
                    if !skip { files.push(FileStatus { path: p, status: s.to_string(), staged: false, additions: 0, deletions: 0 }); }
                }
            }
        }
        Ok(files)
    }

    fn get_diff(&self, path: &str, file: &str) -> Result<String, String> {
        let repo = open(path)?;
        let wd = workdir(&repo)?.to_path_buf();
        let old = get_head_blob(&repo, file).unwrap_or_default();
        let new = std::fs::read(wd.join(file)).unwrap_or_default();
        if is_binary(&old) || is_binary(&new) {
            return Ok(format!("（二进制文件 {}）", file));
        }
        let mut out = String::new();
        out.push_str(&format!("diff --git a/{} b/{}\n", file, file));
        out.push_str("--- a/"); out.push_str(file); out.push('\n');
        out.push_str("+++ b/"); out.push_str(file); out.push('\n');
        out.push_str(&generate_unified_diff(&old, &new)?);
        Ok(out)
    }

    fn stage_all(&self, path: &str) -> Result<(), String> {
        let repo = open(path)?;
        let wd = workdir(&repo)?.to_path_buf();
        let mut file_list = Vec::new();
        walk_worktree(&wd, &wd, &mut file_list)?;
        let mut entries = Vec::new();
        for rel in &file_list {
            let (kind, blob_id) = write_file_blob(&repo, &wd, rel)?;
            entries.push((rel.clone(), kind, blob_id));
        }
        let tree_id = build_tree_from_entries(&repo, &entries)?;
        write_index_from_tree(&repo, &tree_id)
    }

    fn stage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        let repo = open(path)?;
        let wd = workdir(&repo)?.to_path_buf();
        let mut entries = Vec::new();
        for rel in files {
            let (kind, blob_id) = write_file_blob(&repo, &wd, rel)?;
            entries.push((rel.clone(), kind, blob_id));
        }
        let tree_id = build_tree_from_entries(&repo, &entries)?;
        write_index_from_tree(&repo, &tree_id)
    }

    fn unstage_files(&self, path: &str, files: &[String]) -> Result<(), String> {
        if files.is_empty() { return Ok(()); }
        let repo = open(path)?;
        let index_file = repo.open_index().map_err(|e| format!("打开索引失败: {}", e))?;
        let (state, _) = index_file.into_parts();
        let unstage_set: std::collections::HashSet<&str> = files.iter().map(|f| f.as_str()).collect();
        let mut entries: Vec<(String, EntryKind, gix::hash::ObjectId)> = Vec::new();

        state.entries_with_paths_by_filter_map(|path, entry| {
            let p_str = path.to_string();
            if !unstage_set.contains(p_str.as_str()) {
                Some((p_str, entry_mode_to_kind(entry.mode), entry.id))
            } else { None }
        }).for_each(|(_, data)| entries.push(data));

        if let Ok(head) = repo.head_commit() {
            if let Ok(tree) = head.tree() {
                for e in tree.iter().filter_map(Result::ok) {
                    let p = e.filename().to_string();
                    if unstage_set.contains(p.as_str()) {
                        let kind = match e.mode().kind() {
                            EntryKind::Blob => EntryKind::Blob,
                            EntryKind::BlobExecutable => EntryKind::BlobExecutable,
                            EntryKind::Link => EntryKind::Link,
                            _ => continue,
                        };
                        entries.push((p, kind, e.object_id()));
                    }
                }
            }
        }

        let tree_id = build_tree_from_entries(&repo, &entries)?;
        write_index_from_tree(&repo, &tree_id)
    }

    fn commit_with_identity(&self, path: &str, message: &str, name: &str, email: &str) -> Result<CommitInfo, String> {
        let repo = open(path)?;
        self.stage_all(path)?;
        let index_file = repo.open_index().map_err(|e| format!("打开索引失败: {}", e))?;
        let (state, _) = index_file.into_parts();
        let mut entries: Vec<(String, EntryKind, gix::hash::ObjectId)> = Vec::new();
        state.entries_with_paths_by_filter_map(|path, entry| {
            Some((path.to_string(), entry_mode_to_kind(entry.mode), entry.id))
        }).for_each(|(_, data)| entries.push(data));

        let tree_id = if entries.is_empty() {
            repo.empty_tree().id().detach()
        } else {
            build_tree_from_entries(&repo, &entries)?
        };

        let parents: Vec<gix::hash::ObjectId> = match repo.head_id() {
            Ok(id) => vec![id.detach()],
            Err(_) => vec![],
        };

        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let sig_str = format!("{} <{}> {} +0000", name, email, now);
        let sig = gix::actor::SignatureRef::from_bytes(sig_str.as_bytes())
            .map_err(|e| format!("构造签名失败: {}", e))?;

        let commit_id = repo.commit_as(sig, sig, "HEAD", message, tree_id, parents)
            .map_err(|e| format!("提交失败: {}", e))?;

        Ok(CommitInfo {
            hash: commit_id.to_string(),
            short_hash: commit_id.to_string().chars().take(7).collect(),
            message: message.to_string(),
            author: name.to_string(),
            time: now as i64,
        })
    }

    fn get_log(&self, path: &str, limit: u32) -> Result<Vec<CommitInfo>, String> {
        let repo = open(path)?;
        let head = match repo.head_id() { Ok(id) => id, Err(_) => return Ok(Vec::new()) };
        let walk = repo.rev_walk([head.detach()])
            .sorting(gix::revision::walk::Sorting::ByCommitTime(
                gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
            )).all().map_err(|e| e.to_string())?;
        let mut commits = Vec::new();
        for info in walk.take(limit as usize) {
            let info = info.map_err(|e| e.to_string())?;
            let commit = info.object().map_err(|e| e.to_string())?;
            let author = commit.author().map_err(|e| e.to_string())?;
            commits.push(CommitInfo {
                hash: info.id.to_string(),
                short_hash: info.id.to_string().chars().take(7).collect(),
                message: commit.message_raw().map(|m| m.to_string()).unwrap_or_default(),
                author: author.name.to_string(),
                time: info.commit_time(),
            });
        }
        Ok(commits)
    }

    fn get_ahead_behind(&self, path: &str) -> Result<(usize, usize), String> {
        let repo = open(path)?;
        let head = match repo.head_id() { Ok(id) => id, Err(_) => return Ok((0, 0)) };
        let branch = match repo.head_name().map_err(|e| e.to_string())? {
            Some(b) => b, None => return Ok((0, 0)),
        };
        let upstream_name = format!("refs/remotes/origin/{}", branch.shorten());
        let mut upstream = match repo.find_reference(&upstream_name) {
            Ok(r) => r, Err(_) => return Ok((0, 0)),
        };
        let upstream_id = match upstream.peel_to_id() {
            Ok(id) => id, Err(_) => return Ok((0, 0)),
        };
        let ahead = repo.rev_walk([head.detach()])
            .with_hidden([upstream_id.detach()])
            .all().map_err(|e| e.to_string())?.count();
        let behind = repo.rev_walk([upstream_id.detach()])
            .with_hidden([head.detach()])
            .all().map_err(|e| e.to_string())?.count();
        Ok((ahead, behind))
    }

    fn reset_to_commit(&self, path: &str, hash: &str) -> Result<(), String> {
        let repo = open(path)?;
        let wd = workdir(&repo)?.to_path_buf();
        let hash_id = gix::hash::ObjectId::from_hex(hash.as_bytes())
            .map_err(|e| format!("无效的提交哈希: {}", e))?;
        let target = repo.find_commit(hash_id).map_err(|e| format!("查找提交失败: {}", e))?;
        let tree_id = target.tree_id().map_err(|e| e.to_string())?;

        write_tree_to_worktree(&repo, &*tree_id, &wd)?;

        let state = gix::index::State::from_tree(&*tree_id, &repo,
            gix::index::validate::path::component::Options::default(),
        ).map_err(|e| e.to_string())?;

        let tracked: std::collections::HashSet<String> = state.entries_with_paths_by_filter_map(|path, _| Some(path.to_string())).map(|(_, p)| p).collect();
        clean_untracked_files(&wd, &tracked);

        let mut file = gix::index::File::from_state(state, repo.index_path().to_path_buf());
        file.write(gix::index::write::Options::default()).map_err(|e| format!("写入索引失败: {}", e)).map(|_| ())
    }

    fn revert_to_commit(&self, path: &str, hash: &str, message: &str, name: &str, email: &str) -> Result<(), String> {
        self.reset_to_commit(path, hash)?;
        self.commit_with_identity(path, message, name, email)?;
        Ok(())
    }

    fn discard_changes(&self, path: &str) -> Result<(), String> {
        let repo = open(path)?;
        let wd = workdir(&repo)?.to_path_buf();
        let head = repo.head_id().map_err(|_| "没有提交记录".to_string())?;
        let head_commit = head.object().map_err(|e| e.to_string())?.into_commit();
        let tree_id = head_commit.tree_id().map_err(|e| e.to_string())?;

        write_tree_to_worktree(&repo, &*tree_id, &wd)?;

        let state = gix::index::State::from_tree(&*tree_id, &repo,
            gix::index::validate::path::component::Options::default(),
        ).map_err(|e| e.to_string())?;

        let tracked: std::collections::HashSet<String> = state.entries_with_paths_by_filter_map(|path, _| Some(path.to_string())).map(|(_, p)| p).collect();
        clean_untracked_files(&wd, &tracked);

        let mut file = gix::index::File::from_state(state, repo.index_path().to_path_buf());
        file.write(gix::index::write::Options::default()).map_err(|e| format!("写入索引失败: {}", e)).map(|_| ())
    }
}

fn entry_mode_to_kind(mode: gix::index::entry::Mode) -> EntryKind {
    if mode == gix::index::entry::Mode::FILE { EntryKind::Blob }
    else if mode == gix::index::entry::Mode::FILE_EXECUTABLE { EntryKind::BlobExecutable }
    else if mode == gix::index::entry::Mode::SYMLINK { EntryKind::Link }
    else { EntryKind::Blob }
}

fn clean_untracked_files(workdir: &Path, tracked: &std::collections::HashSet<String>) {
    fn clean(dir: &Path, root: &Path, tracked: &std::collections::HashSet<String>, errors: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries {
            let Ok(entry) = entry else { continue };
            let path = entry.path();
            let rel = path.strip_prefix(root).ok()
                .map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();
            if rel == ".git" || rel.starts_with(".git/") { continue; }
            if path.is_dir() {
                clean(&path, root, tracked, errors);
                if std::fs::read_dir(&path).map_or(true, |mut e| e.next().is_none()) {
                    let _ = std::fs::remove_dir(&path);
                }
            } else if !tracked.contains(&rel) {
                if let Err(e) = std::fs::remove_file(&path) {
                    errors.push(format!("删除 {} 失败: {}", rel, e));
                }
            }
        }
    }
    let mut errors = Vec::new();
    clean(workdir, workdir, tracked, &mut errors);
}
