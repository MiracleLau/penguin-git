<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { NButton } from "naive-ui";
import { useAppStore } from "../../stores/appStore";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { useProjectStore } from "../../stores/projectStore";
import { openProject, addRemote } from "../../lib/tauri";

const { t } = useI18n();
const router = useRouter();
const { completeOnboarding } = useAppStore();
const { projectPath, remoteUrl } = useOnboardingStore();
const { setCurrentProject } = useProjectStore();

async function handleEnter() {
  completeOnboarding();
  if (projectPath) {
    try {
      const info = await openProject(projectPath);
      setCurrentProject(info);
    } catch { /* ignore */ }
    if (remoteUrl) {
      try { await addRemote(projectPath, "origin", remoteUrl); } catch { /* ignore */ }
    }
  }
  router.replace(projectPath ? "/dashboard" : "/select");
}
</script>

<template>
  <div style="display: flex; align-items: center; justify-content: center; min-height: 100vh;">
    <div style="text-align: center; display: flex; flex-direction: column; align-items: center; gap: 8px;">
      <h1 style="font-size: 22px; font-weight: 700;">{{ t("onboarding.done.title") }}</h1>
      <p style="opacity: 0.6;">{{ t("onboarding.done.subtitle") }}</p>
      <n-button size="large" type="primary" @click="handleEnter">{{ t("onboarding.done.enter") }}</n-button>
    </div>
  </div>
</template>
