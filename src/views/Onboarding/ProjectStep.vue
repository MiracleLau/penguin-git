<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton, NInput } from "naive-ui";

const { t } = useI18n();
const router = useRouter();
const { projectPath, setProject } = useOnboardingStore();
const path = ref<string | null>(projectPath);

async function handlePickFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: t("onboarding.project.selectFolder"),
  });
  if (selected) {
    const folderName = selected.split("/").pop() || selected.split("\\").pop() || "";
    path.value = selected;
    setProject(selected, folderName);
  }
}
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.project.title") }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <n-input :value="path || ''" readonly :placeholder="t('onboarding.project.placeholder')" />
      <n-button @click="handlePickFolder">{{ t("onboarding.project.selectFolder") }}</n-button>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding/git-check')">{{ t("common.back") }}</n-button>
        <n-button :disabled="!path" @click="router.push('/onboarding/init')">{{ t("common.next") }}</n-button>
      </div>
    </template>
  </n-card>
</template>
