import { defineStore } from "pinia";
import { ref } from "vue";
import type { FileStatus } from "../lib/types";

export const useGitStore = defineStore("git", () => {
  const status = ref<FileStatus[]>([]);
  const selectedFiles = ref<Set<string>>(new Set());
  const history = ref<any[]>([]);
  const statusLoading = ref(false);
  const historyLoading = ref(false);
  const commitMessage = ref("");
  const ahead = ref(0);
  const behind = ref(0);

  function setStatus(s: FileStatus[]) {
    status.value = s;
    selectedFiles.value = new Set(s.filter((f) => f.staged).map((f) => f.path));
  }

  function toggleFile(file: string) {
    const next = new Set(selectedFiles.value);
    if (next.has(file)) next.delete(file);
    else next.add(file);
    selectedFiles.value = next;
  }

  function selectAll() {
    selectedFiles.value = new Set(status.value.map((f) => f.path));
  }

  function deselectAll() {
    selectedFiles.value = new Set();
  }

  function setHistory(h: any[]) {
    history.value = h;
  }

  function setStatusLoading(v: boolean) {
    statusLoading.value = v;
  }

  function setHistoryLoading(v: boolean) {
    historyLoading.value = v;
  }

  function setCommitMessage(msg: string) {
    commitMessage.value = msg;
  }

  function setAheadBehind(a: number, b: number) {
    ahead.value = a;
    behind.value = b;
  }

  return {
    status, selectedFiles, history, statusLoading, historyLoading,
    commitMessage, ahead, behind,
    setStatus, toggleFile, selectAll, deselectAll,
    setHistory, setStatusLoading, setHistoryLoading,
    setCommitMessage, setAheadBehind,
  };
});
