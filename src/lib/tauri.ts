import { invoke } from "@tauri-apps/api/core";
import type {
  FileStatus, CommitInfo,
  SshKeyInfo, ProjectInfo, RecentProject,
  AppSettings, Credential, TestResult,
} from "./types";

// System
export const checkGitInstalled = () => invoke<boolean>("check_git_installed");
export const getGitVersion = () => invoke<string | null>("get_git_version");

// Project
export const openProject = (path: string) => invoke<ProjectInfo>("open_project", { path });
export const isGitRepo = (path: string) => invoke<boolean>("is_git_repo", { path });
export const checkGitignoreExists = (path: string) => invoke<boolean>("check_gitignore_exists", { path });
export const initRepo = (path: string, gitignore?: string) =>
  invoke<void>("init_repo", { path, gitignore: gitignore || null });
export const getRecentProjects = () => invoke<RecentProject[]>("get_recent_projects");
export const saveRecentProject = (path: string, name: string) =>
  invoke<void>("save_recent_project", { path, name });

// SSH
export const checkSshKey = () => invoke<SshKeyInfo>("check_ssh_key");
export const generateSshKey = (comment: string) =>
  invoke<SshKeyInfo>("generate_ssh_key", { comment });
export const getPublicKey = () => invoke<string>("get_public_key");
export const copyPublicKey = () => invoke<string>("copy_public_key");
export const openPlatformSshSettings = (platform: string) =>
  invoke<void>("open_platform_ssh_settings", { platform });
export const testSshConnection = (host: string) =>
  invoke<TestResult>("test_ssh_connection", { host });

// Git Local (via backend)
export const getStatus = (path: string) => invoke<FileStatus[]>("get_status", { path });
export const getDiff = (path: string, file: string) =>
  invoke<string>("get_diff", { path, file });
export const stageFiles = (path: string, files: string[]) =>
  invoke<void>("stage_files", { path, files });
export const stageAll = (path: string) => invoke<void>("stage_all", { path });
export const unstageFiles = (path: string, files: string[]) =>
  invoke<void>("unstage_files", { path, files });
export const commit = (path: string, message: string) =>
  invoke<CommitInfo>("commit", { path, message });
export const getLog = (path: string, limit?: number) =>
  invoke<CommitInfo[]>("get_log", { path, limit: limit || 50 });
export const getAheadBehind = (path: string) =>
  invoke<[number, number]>("get_ahead_behind", { path });
export const resetToCommit = (path: string, hash: string) =>
  invoke<void>("reset_to_commit", { path, hash });
export const revertToCommit = (path: string, hash: string, message: string) =>
  invoke<void>("revert_to_commit", { path, hash, message });
export const discardChanges = (path: string) =>
  invoke<void>("discard_changes", { path });

// File Watcher
export const startWatching = (path: string) =>
  invoke<void>("start_watching", { path });
export const stopWatching = () =>
  invoke<void>("stop_watching");

// Remote Git (CLI)
export const addRemote = (path: string, name: string, url: string) =>
  invoke<void>("add_remote", { path, name, url });
export const removeRemote = (path: string, name: string) =>
  invoke<void>("remove_remote", { path, name });
export const push = (path: string) => invoke<string>("push", { path });
export const pull = (path: string) => invoke<string>("pull", { path });
export const fetch = (path: string) => invoke<void>("fetch", { path });
export const getRemoteUrl = (path: string) => invoke<string | null>("get_remote_url", { path });

// Settings
export const getSettings = () => invoke<AppSettings>("get_settings");
export const setSettings = (s: AppSettings) => invoke<void>("set_settings", { s });
export const getGitGlobalConfig = (key: string) => invoke<string | null>("get_git_global_config", { key });
export const setGitGlobalConfig = (key: string, value: string) =>
  invoke<void>("set_git_global_config", { key, value });
export const detectGitPath = () => invoke<string | null>("detect_git_path");

// Credentials
export const listCredentials = () => invoke<Credential[]>("list_credentials");
export const saveCredential = (url: string, username: string, password: string) =>
  invoke<void>("save_credential", { url, username, password });
export const removeCredential = (url: string) =>
  invoke<void>("remove_credential", { url });
