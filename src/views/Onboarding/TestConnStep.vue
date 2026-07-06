<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { testSshConnection } from "../../lib/tauri";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton, NTag, NSpin } from "naive-ui";

const PLATFORM_HOSTS: Record<string, string> = { github: "github.com", codeberg: "codeberg.org" };

const { t } = useI18n();
const router = useRouter();
const { platform, setConnection } = useOnboardingStore();
const testing = ref(false);
const result = ref<{ success: boolean; message: string } | null>(null);

async function handleTest() {
  testing.value = true;
  const host = PLATFORM_HOSTS[platform || ""] || platform || "github.com";
  const res = await testSshConnection(host);
  result.value = res;
  setConnection(res.success);
  testing.value = false;
}
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.testConn.title") }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <div v-if="testing" style="display: flex; align-items: center; gap: 8px;">
        <n-spin size="small" /> <span style="opacity: 0.6;">{{ t("onboarding.testConn.testing") }}</span>
      </div>
      <n-tag v-else-if="result?.success" type="success">{{ t("onboarding.testConn.success") }}</n-tag>
      <div v-else-if="result && !result.success" style="display: flex; flex-direction: column; gap: 6px;">
        <n-tag type="error">{{ t("onboarding.testConn.fail") }}</n-tag>
        <p style="font-size: 12px; opacity: 0.6;">{{ result.message }}</p>
      </div>
      <n-button type="primary" @click="handleTest" :disabled="testing">
        {{ testing ? t("common.loading") : t("onboarding.testConn.retry") }}
      </n-button>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding/add-key')">{{ t("common.back") }}</n-button>
        <n-button quaternary @click="router.push('/onboarding/done')">
          {{ result?.success ? t("common.next") : t("common.skipStep") }}
        </n-button>
      </div>
    </template>
  </n-card>
</template>
