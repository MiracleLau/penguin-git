<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useI18n } from "vue-i18n";
import { useProjectStore } from "../../stores/projectStore";
import { useGitStore } from "../../stores/gitStore";
import { startWatching, stopWatching, getAheadBehind, push, pull, openProject } from "../../lib/tauri";
import { showSuccess, showDanger } from "../../lib/notify";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { NButton, NTag, NIcon, NLayout, NLayoutHeader, NLayoutContent, NButtonGroup, NTooltip, NSpin } from "naive-ui";
import { RouterView } from "vue-router";

const { t } = useI18n();
const router = useRouter();
const route = useRoute();
const project = useProjectStore();
const git = useGitStore();
const syncing = ref(false);
const msg = ref<string | null>(null);
const dashboardLoading = ref(true);
const activeTab = computed(() => route.path);

const TABS = [
  { path: "/dashboard/changes", labelKey: "dashboard.changes" },
  { path: "/dashboard/history", labelKey: "dashboard.history" },
  { path: "/dashboard/settings", labelKey: "dashboard.settings" },
];

async function checkSync() {
  if (!project.currentProject) return;
  const [a, b] = await getAheadBehind(project.currentProject.path);
  git.setAheadBehind(a, b);
}

async function initProject() {
  if (!project.currentProject) return;
  dashboardLoading.value = true;
  try {
    const info = await openProject(project.currentProject.path);
    project.setCurrentProject(info);
    startWatching(project.currentProject.path);
    await checkSync();
    getCurrentWindow().setTitle(`Penguin Git - ${info.name}`).catch(() => {});
  } finally {
    dashboardLoading.value = false;
  }
}

onMounted(() => {
  initProject();
});

onUnmounted(() => {
  stopWatching();
});

async function handlePush() {
  if (!project.currentProject) return;
  syncing.value = true;
  msg.value = null;
  try {
    const output = await push(project.currentProject.path);
    msg.value = output;
    checkSync();
    showSuccess(output.includes("Everything up-to-date") ? "已是最新" : "推送成功");
  } catch (e) {
    msg.value = String(e);
    showDanger("推送失败", String(e));
  } finally {
    syncing.value = false;
  }
}

async function handlePull() {
  if (!project.currentProject) return;
  syncing.value = true;
  msg.value = null;
  try {
    const output = await pull(project.currentProject.path);
    msg.value = output;
    checkSync();
    showSuccess("拉取成功");
  } catch (e) {
    msg.value = String(e);
    showDanger("拉取失败", String(e));
  } finally {
    syncing.value = false;
  }
}

function goBack() {
  stopWatching();
  project.clearCurrentProject();
  router.replace("/select");
}

const syncTagType = computed(() => git.ahead > 0 ? "warning" : git.behind > 0 ? "error" : "success");
const syncLabel = computed(() =>
  git.ahead > 0 ? t("dashboard.toolbar.ahead", { count: git.ahead }) :
  git.behind > 0 ? t("dashboard.toolbar.behind", { count: git.behind }) :
  t("dashboard.toolbar.synced")
);

function navigate(path: string) {
  router.push(path);
}
</script>

<template>
  <n-layout style="height: 100vh; overflow: hidden;">
    <n-layout-header style="display: flex; align-items: center; gap: 6px; padding: 6px 12px; border-bottom: 1px solid; height: 40px;">
      <n-button circle size="small" quaternary @click="goBack">
        <template #icon><n-icon><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5"/><path d="M12 19l-7-7 7-7"/></svg></n-icon></template>
      </n-button>
      <n-icon size="16" style="opacity: 0.6;"><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/></svg></n-icon>
      <span style="font-size: 14px; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; min-width: 0;">{{ project.currentProject?.name }}</span>
      <div style="flex: 1;" />
      <n-button circle size="small" quaternary title="全局设置" @click="router.push('/settings')">
        <template #icon><n-icon><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.32 9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z"/></svg></n-icon></template>
      </n-button>
      <n-tag :type="syncTagType as any" size="small" round>{{ syncLabel }}</n-tag>
      <n-button-group size="small">
        <n-tooltip trigger="hover">
          <template #trigger>
            <n-button :disabled="syncing || !project.currentProject?.hasRemote || git.ahead === 0" quaternary @click="handlePush">
              <template #icon><n-icon><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19V5"/><path d="M5 12l7-7 7 7"/></svg></n-icon></template>
            </n-button>
          </template>
          {{ t("dashboard.toolbar.pushToRemote") }}
        </n-tooltip>
        <n-tooltip trigger="hover">
          <template #trigger>
            <n-button :disabled="syncing || !project.currentProject?.hasRemote" quaternary @click="handlePull">
              <template #icon><n-icon><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14"/><path d="M19 12l-7 7-7-7"/></svg></n-icon></template>
            </n-button>
          </template>
          {{ t("dashboard.toolbar.pullFromRemote") }}
        </n-tooltip>
      </n-button-group>
    </n-layout-header>

    <div v-if="msg" style="border-bottom: 1px solid; padding: 6px 12px; font-size: 12px; font-family: monospace; max-height: 80px; overflow-y: auto;">{{ msg }}</div>

    <div style="display: flex; border-bottom: 1px solid; padding: 0 8px; gap: 0;">
      <div
        v-for="tab in TABS" :key="tab.path"
        :style="{
          display: 'flex', alignItems: 'center', gap: '4px', padding: '8px 12px',
          fontSize: '13px', cursor: 'pointer', borderBottom: activeTab.startsWith(tab.path) ? '2px solid' : '2px solid transparent',
          fontWeight: activeTab.startsWith(tab.path) ? 600 : 400, userSelect: 'none',
        }"
        @click="navigate(tab.path)"
      >
        {{ t(tab.labelKey) }}
      </div>
    </div>

    <n-layout-content style="flex: 1; overflow: auto; position: relative;">
      <n-spin :show="dashboardLoading" style="height: 100%;">
        <router-view />
      </n-spin>
    </n-layout-content>
  </n-layout>
</template>
