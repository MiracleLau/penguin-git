<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { openPlatformSshSettings } from "../../lib/tauri";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton } from "naive-ui";

const PLATFORM_LABELS: Record<string, string> = { github: "GitHub", codeberg: "Codeberg", gitea: "Gitea" };

const { t } = useI18n();
const router = useRouter();
const { platform, setKeyAdded } = useOnboardingStore();
const platformLabel = PLATFORM_LABELS[platform || ""] || platform || "GitHub";

function handleOpen() {
  openPlatformSshSettings(platform || "github");
}
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.addKey.title", { platform: platformLabel }) }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <p style="font-size: 14px; opacity: 0.7;">{{ t("onboarding.addKey.instruction") }}</p>
      <n-button @click="handleOpen">{{ t("onboarding.addKey.openPage", { platform: platformLabel }) }}</n-button>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding/remote')">{{ t("common.back") }}</n-button>
        <div style="display: flex; gap: 6px;">
          <n-button quaternary @click="router.push('/onboarding/done')">{{ t("common.skipStep") }}</n-button>
          <n-button @click="() => { setKeyAdded(true); router.push('/onboarding/test-conn'); }">
            {{ t("onboarding.addKey.done") }}
          </n-button>
        </div>
      </div>
    </template>
  </n-card>
</template>
