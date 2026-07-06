export interface FileStatus {
  path: string;
  status: string;
  staged: boolean;
  additions: number;
  deletions: number;
}

export interface CommitInfo {
  hash: string;
  shortHash: string;
  message: string;
  author: string;
  time: number;
}

export interface CommitDetail {
  hash: string;
  message: string;
  author: string;
  email: string;
  time: number;
  files: FileChange[];
}

export interface FileChange {
  path: string;
  status: string;
  additions: number;
  deletions: number;
}

export interface SshKeyInfo {
  exists: boolean;
  publicKey: string | null;
  path: string | null;
}

export interface ProjectInfo {
  path: string;
  name: string;
  isGitRepo: boolean;
  hasRemote: boolean;
  remoteUrl: string | null;
}

export interface RecentProject {
  path: string;
  name: string;
  lastOpened: number;
}

export interface TestResult {
  success: boolean;
  message: string;
}

export interface AppSettings {
  gitImpl?: string | null;
  gitPath?: string | null;
  gitName?: string | null;
  gitEmail?: string | null;
}

export interface Credential {
  url: string;
  username: string;
  password?: string | null;
}
