<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import { getStatus, stageFiles, stageAll, unstageFiles, commit, getDiff } from "../../lib/tauri";
import { useProjectStore } from "../../stores/projectStore";
import { useGitStore } from "../../stores/gitStore";
import { listen } from "@tauri-apps/api/event";
import { showSuccess, showDanger } from "../../lib/notify";
import { parseDiff } from "../../lib/diffParser";
import DiffView from "../../components/DiffView.vue";
import { NButton, NInput, NSpin, NTag, NEmpty, NIcon, NList, NListItem, NModal } from "naive-ui";
import { AddOutline, RemoveOutline } from "@vicons/ionicons5";

const { t } = useI18n();
const project = useProjectStore();
const git = useGitStore();
const loading = ref(true);
const committing = ref(false);
const showDiffModal = ref(false);
const diffContent = ref<string | null>(null);
const diffFile = ref<string | null>(null);
const diffRows = computed(() => diffContent.value ? parseDiff(diffContent.value) : []);

async function refresh(silent = false) {
  if (!project.currentProject) return;
  if (!silent) { loading.value = true; git.setStatusLoading(true); }
  try {
    const files = await getStatus(project.currentProject.path);
    git.setStatus(files);
  } catch (e) {
    if (!silent) showDanger("加载失败", String(e));
  } finally {
    if (!silent) { loading.value = false; git.setStatusLoading(false); }
  }
}

onMounted(() => {
  refresh();
  const unlisten = listen("file-changed", () => refresh(true));
  onUnmounted(() => { unlisten.then((fn) => fn()); });
});

async function loadDiff(file: string) {
  if (!project.currentProject) return;
  const diff = await getDiff(project.currentProject.path, file);
  diffContent.value = diff || "（无差异）";
  diffFile.value = file;
}

async function handleStageAll() {
  if (!project.currentProject) return;
  await stageAll(project.currentProject.path);
  refresh();
}

async function handleUnstage(files: string[]) {
  if (!project.currentProject || files.length === 0) return;
  await unstageFiles(project.currentProject.path, files);
  refresh();
}

async function handleStage(file: string) {
  if (!project.currentProject) return;
  await stageFiles(project.currentProject.path, [file]);
  refresh();
}

async function handleCommit() {
  if (!project.currentProject || !git.commitMessage.trim()) return;
  committing.value = true;
  try {
    await commit(project.currentProject.path, git.commitMessage.trim());
    git.setCommitMessage("");
    refresh();
    showSuccess("变更已保存");
  } catch (e) {
    showDanger("保存失败", String(e));
  } finally {
    committing.value = false;
  }
}

async function handleClickFile(file: string) {
  await loadDiff(file);
  showDiffModal.value = true;
}

function closeDiff() {
  showDiffModal.value = false;
  diffContent.value = null;
  diffFile.value = null;
}

const unstagedFiles = computed(() => git.status.filter((f) => !f.staged));
const stagedFiles = computed(() => git.status.filter((f) => f.staged));

function statusLabel(s: string) {
  switch (s) {
    case "new": return t("dashboard.changesView.newFile");
    case "modified": return t("dashboard.changesView.modified");
    case "deleted": return t("dashboard.changesView.deleted");
    case "renamed": return t("dashboard.changesView.renamed");
    default: return s;
  }
}
</script>

<template>
  <div style="padding: 16px; max-width: 720px; margin: 0 auto; display: flex; flex-direction: column; gap: 12px;">
    <n-spin :show="loading">
      <div style="display: flex; align-items: center; justify-content: space-between; flex-shrink: 0;">
      <h2 style="font-size: 16px; font-weight: 600;">{{ t("dashboard.changesView.title") }}</h2>
      <div style="display: flex; gap: 8px;">
        <n-button v-if="unstagedFiles.length > 0" size="small" quaternary @click="handleStageAll">
          {{ t("dashboard.changesView.stageAll") }}
        </n-button>
        <n-button size="small" quaternary :disabled="loading" @click="refresh()">
          <template #icon>
            <n-icon><svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 2v6h-6M3 12a9 9 0 0115-6.7L21 8M3 22v-6h6M21 12a9 9 0 01-15 6.7L3 16"/></svg></n-icon>
          </template>
        </n-button>
      </div>
    </div>

    <div style="max-height: calc(100vh - 230px); overflow-y: auto;">
        <n-empty v-if="!loading && git.status.length === 0" :description="t('dashboard.changesView.noChanges')" style="padding: 32px 0;">
          <template #icon>
            <n-icon size="48" style="opacity: 0.3;"><svg viewBox="0 0 24 24" width="48" height="48" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/></svg></n-icon>
          </template>
        </n-empty>

        <div v-else style="display: flex; flex-direction: column; gap: 8px;">
          <div v-if="stagedFiles.length > 0">
            <p style="font-size: 12px; font-weight: 600; opacity: 0.5; text-transform: uppercase; letter-spacing: 0.5px; padding: 0 4px 4px;">已暂存</p>
            <n-list>
              <n-list-item v-for="f in stagedFiles" :key="'s-'+f.path" clickable @click="handleClickFile(f.path)">
                <div style="display: flex; align-items: center; gap: 8px; width: 100%;">
                  <span style="font-size: 12px; font-family: monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; flex: 1;">{{ f.path }}</span>
                  <n-tag size="tiny">{{ statusLabel(f.status) }}</n-tag>
                  <n-button size="tiny" quaternary circle title="取消暂存" @click.stop="handleUnstage([f.path])">
                    <template #icon><n-icon><RemoveOutline /></n-icon></template>
                  </n-button>
                </div>
              </n-list-item>
            </n-list>
          </div>

          <div v-if="unstagedFiles.length > 0">
            <p style="font-size: 12px; font-weight: 600; opacity: 0.5; text-transform: uppercase; letter-spacing: 0.5px; padding: 0 4px 4px;">未暂存</p>
            <n-list>
              <n-list-item v-for="f in unstagedFiles" :key="'u-'+f.path" clickable @click="handleClickFile(f.path)">
                <div style="display: flex; align-items: center; gap: 8px; width: 100%;">
                  <span style="font-size: 12px; font-family: monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; flex: 1;">{{ f.path }}</span>
                  <n-tag size="tiny">{{ statusLabel(f.status) }}</n-tag>
                  <n-button size="tiny" quaternary circle title="暂存" @click.stop="handleStage(f.path)">
                    <template #icon><n-icon><AddOutline /></n-icon></template>
                  </n-button>
                </div>
              </n-list-item>
            </n-list>
          </div>
        </div>
    </div>

    <div style="display: flex; flex-direction: column; gap: 8px;">
      <n-input v-model:value="git.commitMessage" :placeholder="t('dashboard.changesView.commitMessage')" />
      <n-button type="primary" @click="handleCommit" :disabled="!git.commitMessage.trim() || git.status.length === 0 || committing">
        {{ committing ? t("dashboard.changesView.saving") : t("dashboard.changesView.save") }}
      </n-button>
    </div>
    </n-spin>

    <n-modal v-model:show="showDiffModal" preset="card" :title="diffFile" style="max-width: 960px;" @close="closeDiff" @mask-click="closeDiff">
      <DiffView :rows="diffRows" />
    </n-modal>
  </div>
</template>
