<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import { getRecentProjects, isGitRepo, checkGitignoreExists, initRepo } from "../lib/tauri";
import { useProjectStore } from "../stores/projectStore";
import { GITIGNORE_TEMPLATES } from "../lib/gitignoreTemplates";
import type { RecentProject } from "../lib/types";
import { NButton, NCard, NDivider, NList, NListItem, NEmpty, NSpin, NIcon, NScrollbar, NTooltip, NModal, NCheckbox, NCheckboxGroup, NSpace } from "naive-ui";
import { FolderOpenOutline, SettingsOutline } from "@vicons/ionicons5";

const TEMPLATE_KEYS = Object.keys(GITIGNORE_TEMPLATES);

const { t, locale } = useI18n();
const router = useRouter();
const projectStore = useProjectStore();
const loading = ref(true);
const showInitDialog = ref(false);
const pendingInitPath = ref("");
let pendingInitResolve: ((v: boolean) => void) | null = null;
const showGitignoreSection = ref(false);
const selectedTemplates = ref<string[]>(["general"]);

onMounted(async () => {
  try {
    const projects = await getRecentProjects();
    projectStore.setRecentProjects(projects);
  } catch (e) { console.error("加载最近项目失败:", e); }
  loading.value = false;
});

async function handleOpenFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: t("projectSelect.openFolder"),
  });
  if (!selected) return;
  if (!(await isGitRepo(selected))) {
    if (!(await ensureGitRepo(selected))) return;
  }
  projectStore.setCurrentProject({
    path: selected, name: selected.replace(/\\/g, '/').split('/').filter(Boolean).pop() || selected,
    isGitRepo: true, hasRemote: false, remoteUrl: null,
  });
  router.replace("/dashboard");
}

async function handleOpenRecent(p: RecentProject) {
  if (!(await isGitRepo(p.path))) {
    if (!(await ensureGitRepo(p.path))) return;
  }
  projectStore.setCurrentProject({
    path: p.path, name: p.name,
    isGitRepo: true, hasRemote: false, remoteUrl: null,
  });
  router.replace("/dashboard");
}

async function ensureGitRepo(path: string): Promise<boolean> {
  pendingInitPath.value = path;
  showGitignoreSection.value = !(await checkGitignoreExists(path));
  selectedTemplates.value = ["general"];
  showInitDialog.value = true;
  return new Promise((resolve) => {
    pendingInitResolve = resolve;
  });
}

async function handleInitConfirm() {
  try {
    let content: string | undefined;
    if (showGitignoreSection.value && selectedTemplates.value.length > 0) {
      content = selectedTemplates.value
        .map((k) => (GITIGNORE_TEMPLATES as any)[k])
        .filter(Boolean)
        .join("\n");
    }
    await initRepo(pendingInitPath.value, content || undefined);
    showInitDialog.value = false;
    pendingInitResolve?.(true);
  } catch {
    pendingInitResolve?.(false);
  } finally {
    pendingInitResolve = null;
  }
}

function handleInitCancel() {
  showInitDialog.value = false;
  pendingInitResolve?.(false);
  pendingInitResolve = null;
}

function formatTime(ts: number) {
  const diff = Date.now() / 1000 - ts;
  if (diff < 60) return t("common.justNow");
  if (diff < 3600) return t("common.minutesAgo", { n: Math.floor(diff / 60) });
  if (diff < 86400) return t("common.hoursAgo", { n: Math.floor(diff / 3600) });
  return t("common.daysAgo", { n: Math.floor(diff / 86400) });
}
</script>

<template>
  <div style="height: 100vh; overflow: hidden; display: flex; align-items: center; justify-content: center; padding: 16px; position: relative;">
    <n-button circle quaternary style="position: absolute; top: 16px; right: 16px;" @click="router.push('/settings')">
      <template #icon>
        <n-icon><SettingsOutline /></n-icon>
      </template>
    </n-button>
    <n-card style="width: 100%; max-width: 400px; max-height: calc(100vh - 32px);">
      <template #header>
          <div style="text-align: center; padding: 12px 0;">
            <img src="/icon.png" style="width: 48px; height: 48px; border-radius: 8px;" alt="Penguin Git" />
            <h1 style="font-size: 20px; font-weight: 700; margin-top: 8px;">Penguin Git</h1>
          </div>
      </template>
      <n-button size="large" block type="primary" @click="handleOpenFolder">
        <template #icon><n-icon><FolderOpenOutline /></n-icon></template>
        {{ t("projectSelect.openFolder") }}
      </n-button>
      <n-divider>{{ t("projectSelect.recent") }}</n-divider>
      <n-spin :show="loading">
        <n-empty v-if="!loading && projectStore.recentProjects.length === 0" :description="t('projectSelect.empty')" />
        <n-scrollbar v-else-if="projectStore.recentProjects.length > 0" style="max-height: 240px;">
          <n-list>
            <n-list-item v-for="p in projectStore.recentProjects" :key="p.path" clickable @click="handleOpenRecent(p)" style="cursor: pointer;">
              <template #prefix><n-icon><FolderOpenOutline /></n-icon></template>
              <div style="display: flex; align-items: center; gap: 8px; width: 100%;">
                <span style="font-size: 14px; font-weight: 500; white-space: nowrap; flex-shrink: 0;">{{ p.name }}</span>
                <n-tooltip trigger="hover" style="min-width: 0; flex: 1;">
                  <template #trigger>
                    <span style="font-size: 12px; opacity: 0.6; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; display: block;">{{ p.path }}</span>
                  </template>
                  {{ p.path }}
                </n-tooltip>
                <span style="font-size: 12px; opacity: 0.5; white-space: nowrap; flex-shrink: 0;">{{ formatTime(p.lastOpened) }}</span>
              </div>
            </n-list-item>
          </n-list>
        </n-scrollbar>
      </n-spin>
    </n-card>
  </div>

    <n-modal v-model:show="showInitDialog" preset="card" title="初始化 Git 仓库" style="max-width: 420px;" closable @close="handleInitCancel" @mask-click="handleInitCancel">
      <p style="font-size: 14px; opacity: 0.7; margin-bottom: 16px;">
        所选目录不是 Git 仓库，是否初始化？
      </p>
      <template v-if="showGitignoreSection">
        <n-divider />
        <p style="font-size: 14px; font-weight: 500; margin-bottom: 8px;">{{ t("projectSelect.initGitignore") }}</p>
        <n-checkbox-group v-model:value="selectedTemplates">
          <n-space style="flex-wrap: wrap; gap: 8px;">
            <n-checkbox v-for="key in TEMPLATE_KEYS" :key="key" :value="key" :label="key.toUpperCase()" />
          </n-space>
        </n-checkbox-group>
      </template>
      <div style="display: flex; justify-content: flex-end; gap: 6px; margin-top: 16px;">
        <n-button quaternary @click="handleInitCancel">取消</n-button>
        <n-button type="primary" @click="handleInitConfirm">初始化</n-button>
      </div>
    </n-modal>
</template>
