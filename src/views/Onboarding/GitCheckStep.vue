<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { checkGitInstalled, getGitVersion } from "../../lib/tauri";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton, NTag, NSpin } from "naive-ui";

const { t } = useI18n();
const router = useRouter();
const { setGit } = useOnboardingStore();
const loading = ref(true);
const installed = ref<boolean | null>(null);
const version = ref<string | null>(null);

async function check() {
  loading.value = true;
  const found = await checkGitInstalled();
  const ver = found ? await getGitVersion() : null;
  installed.value = found;
  version.value = ver;
  setGit(found, ver);
  loading.value = false;
}

onMounted(check);
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.gitCheck.title") }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <div v-if="loading" style="display: flex; align-items: center; gap: 8px;">
        <n-spin size="small" /> <span style="opacity: 0.6;">{{ t("common.loading") }}</span>
      </div>
      <n-tag v-else-if="installed" type="success">{{ t("onboarding.gitCheck.found", { version }) }}</n-tag>
      <n-tag v-else type="error">{{ t("onboarding.gitCheck.notFound") }}</n-tag>
      <n-button size="small" quaternary @click="check" :disabled="loading">{{ t("onboarding.gitCheck.recheck") }}</n-button>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding')">{{ t("common.back") }}</n-button>
        <div style="display: flex; gap: 8px;">
          <n-button quaternary @click="router.push('/onboarding/ssh')">{{ t("common.skipStep") }}</n-button>
          <n-button type="primary" :disabled="!installed" @click="router.push('/onboarding/ssh')">{{ t("common.next") }}</n-button>
        </div>
      </div>
    </template>
  </n-card>
</template>
