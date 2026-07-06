<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import { getLog, getStatus, getAheadBehind, resetToCommit, revertToCommit, discardChanges } from "../../lib/tauri";
import { useProjectStore } from "../../stores/projectStore";
import { useGitStore } from "../../stores/gitStore";
import type { CommitInfo } from "../../lib/types";
import { showSuccess, showDanger } from "../../lib/notify";
import { NTag, NSpin, NButton, NModal, NEmpty, NIcon, NDropdown, NTimeline, NTimelineItem } from "naive-ui";

const { t } = useI18n();
const project = useProjectStore();
const git = useGitStore();
const commits = ref<CommitInfo[]>([]);
const loading = ref(true);

// Context menu
const showCtx = ref(false);
const ctxX = ref(0);
const ctxY = ref(0);
const ctxCommit = ref<CommitInfo | null>(null);

// Confirm modal
const showConfirm = ref(false);
const confirmTitle = ref("");
const confirmMsg = ref("");
const confirmType = ref<"safe" | "hard">("safe");
const targetCommit = ref<{ repoPath: string; hash: string; shortHash: string; message: string } | null>(null);

async function loadLog() {
  if (!project.currentProject) return;
  loading.value = true;
  try {
    const log = await getLog(project.currentProject.path, 50);
    commits.value = log;
  } catch {
    commits.value = [];
  }
  loading.value = false;
}

async function refreshAll() {
  if (!project.currentProject) return;
  await loadLog();
  try {
    const files = await getStatus(project.currentProject.path);
    git.setStatus(files);
    const [a, b] = await getAheadBehind(project.currentProject.path);
    git.setAheadBehind(a, b);
  } catch { /* non-critical */ }
}

onMounted(loadLog);

function isHead(c: CommitInfo) {
  return commits.value.length > 0 && c.hash === commits.value[0].hash;
}

function formatTime(ts: number) {
  return new Date(ts * 1000).toLocaleString();
}

function handleContextMenu(e: MouseEvent, c: CommitInfo) {
  e.preventDefault();
  ctxCommit.value = c;
  ctxX.value = e.clientX;
  ctxY.value = e.clientY;
  showCtx.value = true;
}

function closeCtx() {
  showCtx.value = false;
}

const dropdownOptions = computed(() => {
  if (!ctxCommit.value) return [];
  return [
    { label: ctxCommit.value.shortHash, key: "header", disabled: true },
    { type: "divider" as const },
    {
      label: t("dashboard.historyView.safeRevert"),
      key: "safe",
      disabled: isHead(ctxCommit.value),
    },
    {
      label: t("dashboard.historyView.hardRevert"),
      key: "hard",
    },
  ];
});

function handleCtxSelect(key: string) {
  if (!project.currentProject || !ctxCommit.value) return;
  const c = ctxCommit.value;
  targetCommit.value = { repoPath: project.currentProject.path, hash: c.hash, shortHash: c.shortHash, message: c.message };

  if (key === "safe") {
    confirmTitle.value = t("dashboard.historyView.safeRevert");
    confirmMsg.value = t("dashboard.historyView.safeRevertConfirm", { hash: c.shortHash });
    confirmType.value = "safe";
    showConfirm.value = true;
  } else if (key === "hard") {
    confirmTitle.value = t("dashboard.historyView.hardRevert");
    confirmMsg.value = t("dashboard.historyView.hardRevertConfirm", { hash: c.shortHash });
    confirmType.value = "hard";
    showConfirm.value = true;
  }
  closeCtx();
}

async function handleSafeRevert() {
  if (!targetCommit.value) return;
  const { repoPath, hash, shortHash, message } = targetCommit.value;
  try {
    await discardChanges(repoPath);
    await revertToCommit(repoPath, hash, `回退到 ${shortHash}: ${message}`);
    showSuccess(t("dashboard.historyView.revertSuccess", { hash: shortHash }));
    refreshAll();
  } catch (e) {
    showDanger(t("dashboard.historyView.revertFailed"), String(e));
  }
  targetCommit.value = null;
  showConfirm.value = false;
}

async function handleHardReset() {
  if (!targetCommit.value) return;
  const { repoPath, hash, shortHash } = targetCommit.value;
  try {
    await resetToCommit(repoPath, hash);
    showSuccess(t("dashboard.historyView.revertSuccess", { hash: shortHash }));
    refreshAll();
  } catch (e) {
    showDanger(t("dashboard.historyView.revertFailed"), String(e));
  }
  targetCommit.value = null;
  showConfirm.value = false;
}
</script>

<template>
  <div style="padding: 16px; max-width: 720px; margin: 0 auto;">
    <h2 style="font-size: 16px; font-weight: 600; margin-bottom: 12px;">{{ t("dashboard.historyView.title") }}</h2>

    <n-spin :show="loading">
      <n-empty v-if="!loading && commits.length === 0" :description="t('dashboard.historyView.noCommits')">
        <template #icon><n-icon size="48" style="opacity: 0.3;"><svg viewBox="0 0 24 24" width="48" height="48" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="3"/><path d="M12 2v4m0 12v4m10-10h-4M6 12H2"/></svg></n-icon></template>
      </n-empty>

      <n-timeline v-else>
        <n-timeline-item
          v-for="(c, idx) in commits" :key="c.hash"
          :type="idx === 0 ? 'success' : 'default'"
          :time="formatTime(c.time)"
          style="cursor: context-menu;"
          @contextmenu.prevent="handleContextMenu($event, c)"
        >
          <template #header>
            <div style="display: flex; align-items: center; gap: 6px; width: 100%;">
              <span style="font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">{{ c.message }}</span>
              <n-tag v-if="idx === 0" size="tiny" type="success">{{ t("dashboard.historyView.currentVersion") }}</n-tag>
            </div>
          </template>
          <div style="display: flex; justify-content: space-between; align-items: center; font-size: 12px; opacity: 0.6;">
            <span>{{ c.author }}</span>
            <n-tag size="tiny" style="font-family: monospace; flex-shrink: 0;">{{ c.shortHash }}</n-tag>
          </div>
        </n-timeline-item>
      </n-timeline>
    </n-spin>

    <!-- Context menu -->
    <n-dropdown
      placement="bottom-start"
      trigger="manual"
      :x="ctxX"
      :y="ctxY"
      :options="dropdownOptions"
      :show="showCtx"
      @clickoutside="closeCtx"
      @select="handleCtxSelect"
    />

    <!-- Confirm modal -->
    <n-modal v-model:show="showConfirm" preset="card" :title="confirmTitle" style="max-width: 480px;" closable>
      <p style="font-size: 14px; opacity: 0.7; margin-bottom: 16px;">{{ confirmMsg }}</p>
      <div style="display: flex; justify-content: flex-end; gap: 8px;">
        <n-button quaternary @click="showConfirm = false">{{ t("common.cancel") }}</n-button>
        <n-button :type="confirmType === 'hard' ? 'error' : 'primary'" @click="confirmType === 'safe' ? handleSafeRevert() : handleHardReset()">
          {{ t("dashboard.historyView.confirm") }}
        </n-button>
      </div>
    </n-modal>
  </div>
</template>
