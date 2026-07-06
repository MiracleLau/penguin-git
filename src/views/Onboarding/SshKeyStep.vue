<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { checkSshKey, generateSshKey, copyPublicKey, testSshConnection } from "../../lib/tauri";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton, NTag, NSpin } from "naive-ui";

const PLATFORMS = [
  { id: "github", label: "GitHub", host: "github.com" },
  { id: "codeberg", label: "Codeberg", host: "codeberg.org" },
];
const SSH_SETTINGS_URLS: Record<string, string> = {
  github: "https://github.com/settings/keys",
  codeberg: "https://codeberg.org/user/settings/keys",
};

const { t } = useI18n();
const router = useRouter();
const { setSshKey } = useOnboardingStore();
const loading = ref(true);
const generating = ref(false);
const keyData = ref<{ exists: boolean; publicKey: string | null }>({ exists: false, publicKey: null });
const copied = ref(false);
const testing = ref<Record<string, boolean>>({ github: false, codeberg: false });
const testResults = ref<Record<string, { success: boolean; message: string } | null>>({ github: null, codeberg: null });
const anyTesting = computed(() => testing.value.github || testing.value.codeberg);

async function check() {
  loading.value = true;
  const info = await checkSshKey();
  keyData.value = info;
  setSshKey(info.exists, info.publicKey || null);
  loading.value = false;
}

onMounted(async () => {
  await check();
  if (keyData.value.exists) {
    autoTestConnections();
  }
});

async function autoTestConnections() {
  testing.value = { github: true, codeberg: true };
  testResults.value = { github: null, codeberg: null };

  await Promise.all(PLATFORMS.map(async (p) => {
    try {
      const res = await testSshConnection(p.host);
      testResults.value[p.id] = res;
    } catch (e) {
      testResults.value[p.id] = { success: false, message: String(e) };
    }
    testing.value[p.id] = false;
  }));
}

async function handleGenerate() {
  generating.value = true;
  try {
    const info = await generateSshKey(`penguin-git@${navigator.userAgent || "computer"}`);
    keyData.value = info;
    setSshKey(info.exists, info.publicKey || null);
    if (info.exists) {
      await autoTestConnections();
    }
  } catch { } finally {
    generating.value = false;
  }
}

async function handleCopy() {
  if (keyData.value.publicKey) {
    await copyPublicKey();
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 2000);
  }
}
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.ssh.title") }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <template v-if="keyData.exists">
        <n-tag type="success">{{ t("onboarding.ssh.found") }}</n-tag>
      </template>
      <template v-else-if="!loading">
        <n-tag type="warning">{{ t("onboarding.ssh.notFound") }}</n-tag>
      </template>
      <span v-else style="font-size: 13px; opacity: 0.6;">{{ t("onboarding.ssh.checking") }}</span>
      <template v-if="keyData.publicKey">
        <pre style="font-size: 12px; background: rgba(128,128,128,0.08); padding: 8px; border-radius: 12px; overflow-x: auto; white-space: pre-wrap; word-break: break-all; max-height: 72px;">{{ keyData.publicKey }}</pre>
        <n-button size="small" quaternary @click="handleCopy">{{ copied ? t("onboarding.ssh.copied") : t("onboarding.ssh.copy") }}</n-button>
      </template>
      <n-button v-if="!keyData.exists && !loading" type="primary" @click="handleGenerate" :disabled="generating">
        {{ generating ? t("onboarding.ssh.generating") : t("onboarding.ssh.generate") }}
      </n-button>
      <div v-if="keyData.exists" style="margin-top: 8px;">
        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 6px;">
          <p style="font-size: 13px; font-weight: 500;">{{ t("onboarding.ssh.connectionTest") }}</p>
          <n-button v-if="!anyTesting" size="tiny" quaternary @click="autoTestConnections">{{ t("onboarding.ssh.retest") }}</n-button>
        </div>
        <div v-for="p in PLATFORMS" :key="p.id" style="display: flex; align-items: center; gap: 8px; padding: 3px 0;">
          <span style="font-size: 14px; min-width: 84px;">{{ p.label }}</span>
          <n-spin v-if="testing[p.id]" size="small" />
          <span v-if="testing[p.id]" style="font-size: 12px; opacity: 0.6;">{{ t("onboarding.ssh.detecting") }}</span>
          <n-tag v-else-if="testResults[p.id]" :type="testResults[p.id]!.success ? 'success' : 'error'" size="small">
            {{ testResults[p.id]!.success ? t("onboarding.ssh.accessible") : t("onboarding.ssh.inaccessible") }}
          </n-tag>
          <a v-if="testResults[p.id] && !testResults[p.id]!.success"
             :href="SSH_SETTINGS_URLS[p.id]" target="_blank"
             style="font-size: 12px; white-space: nowrap;">配置</a>
        </div>
      </div>
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding/git-check')">{{ t("common.back") }}</n-button>
        <div style="display: flex; gap: 8px;">
          <n-button quaternary @click="router.push('/onboarding/done')">{{ t("common.skipStep") }}</n-button>
          <n-button type="primary" :disabled="!keyData.exists || anyTesting" @click="router.push('/onboarding/done')">{{ t("common.next") }}</n-button>
        </div>
      </div>
    </template>
  </n-card>
</template>
