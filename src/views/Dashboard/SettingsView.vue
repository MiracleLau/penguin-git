<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { getRemoteUrl, addRemote, removeRemote } from "../../lib/tauri";
import { useProjectStore } from "../../stores/projectStore";
import { showSuccess, showDanger } from "../../lib/notify";
import { NCard, NButton, NInput } from "naive-ui";

const { t } = useI18n();
const router = useRouter();
const project = useProjectStore();
const remoteUrl = ref("");
const newRemoteUrl = ref("");

onMounted(async () => {
  if (project.currentProject) {
    const url = await getRemoteUrl(project.currentProject.path);
    remoteUrl.value = url || "";
  }
});

async function reload() {
  if (!project.currentProject) return;
  const url = await getRemoteUrl(project.currentProject.path);
  remoteUrl.value = url || "";
  newRemoteUrl.value = "";
}

async function handleUpdateRemote() {
  if (!project.currentProject || !newRemoteUrl.value.trim()) return;
  try {
    await addRemote(project.currentProject.path, "origin", newRemoteUrl.value.trim());
    showSuccess("远程仓库已更新");
    reload();
  } catch (e) { showDanger("更新失败: " + String(e)); }
}

async function handleRemoveRemote() {
  if (!project.currentProject) return;
  try {
    await removeRemote(project.currentProject.path, "origin");
    showSuccess("远程仓库已移除");
    reload();
  } catch (e) { showDanger("移除失败: " + String(e)); }
}
</script>

<template>
  <div style="padding: 16px; max-width: 720px; margin: 0 auto; display: flex; flex-direction: column; gap: 12px;">
    <div style="display: flex; align-items: center; justify-content: space-between;">
      <h2 style="font-size: 16px; font-weight: 600;">{{ t("dashboard.settingsView.title") }}</h2>
      <n-button size="small" quaternary @click="router.push('/settings')">全局设置</n-button>
    </div>

    <n-card>
      <template #header><h3 style="font-size: 13px; font-weight: 500;">{{ t("dashboard.settingsView.project") }}</h3></template>
      <div style="display: flex; flex-direction: column; gap: 8px; font-size: 14px;">
        <div><span style="opacity: 0.6;">{{ t("dashboard.settingsView.path") }}: </span><span style="font-family: monospace;">{{ project.currentProject?.path }}</span></div>
        <div><span style="opacity: 0.6;">{{ t("dashboard.settingsView.remote") }}: </span><span>{{ remoteUrl || t("dashboard.settingsView.noRemote") }}</span></div>
        <n-button v-if="remoteUrl" size="small" quaternary @click="handleRemoveRemote">移除远程仓库</n-button>
        <div style="display: flex; gap: 8px;">
          <n-input v-model:value="newRemoteUrl" :placeholder="t('dashboard.settingsView.remote')" />
          <n-button type="primary" @click="handleUpdateRemote" :disabled="!newRemoteUrl.trim()">
            {{ remoteUrl ? "更新" : "添加" }}
          </n-button>
        </div>
      </div>
    </n-card>
  </div>
</template>
