<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { isGitRepo, initRepo } from "../../lib/tauri";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { GITIGNORE_TEMPLATES } from "../../lib/gitignoreTemplates";
import { NCard, NButton, NTag, NSpin, NCheckbox, NCheckboxGroup, NSpace } from "naive-ui";

const TEMPLATE_KEYS = Object.keys(GITIGNORE_TEMPLATES);

const { t } = useI18n();
const router = useRouter();
const { projectPath, setInit } = useOnboardingStore();
const loading = ref(true);
const alreadyRepo = ref(false);
const done = ref(false);
const initializing = ref(false);
const selectedTemplates = ref(["general"]);

onMounted(async () => {
  if (!projectPath) return;
  const repo = await isGitRepo(projectPath);
  alreadyRepo.value = repo;
  done.value = repo;
  setInit(repo, null);
  loading.value = false;
});

async function handleInit() {
  if (!projectPath) return;
  initializing.value = true;
  const gitignore = selectedTemplates.value
    .map((k: string) => (GITIGNORE_TEMPLATES as any)[k])
    .filter(Boolean)
    .join("\n");
  try {
    await initRepo(projectPath, gitignore || undefined);
    done.value = true;
    setInit(true, gitignore || null);
  } catch { } finally {
    initializing.value = false;
  }
}
</script>

<template>
  <n-spin :show="loading">
    <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
      <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.init.title") }}</h2></template>
      <div style="display: flex; flex-direction: column; gap: 8px;">
        <n-tag v-if="alreadyRepo" type="success">{{ t("onboarding.init.alreadyRepo") }}</n-tag>
        <n-tag v-else-if="done" type="success">{{ t("onboarding.init.initSuccess") }}</n-tag>
        <template v-else>
          <p style="opacity: 0.7; font-size: 14px;">{{ t("onboarding.init.notRepo") }}</p>
          <div>
            <p style="font-size: 14px; font-weight: 500;">{{ t("onboarding.init.gitignore") }}</p>
            <n-checkbox-group v-model:value="selectedTemplates">
              <n-space style="flex-wrap: wrap; gap: 8px; margin-top: 6px;">
                <n-checkbox v-for="key in TEMPLATE_KEYS" :key="key" :value="key" :label="key.toUpperCase()" />
              </n-space>
            </n-checkbox-group>
          </div>
          <n-button @click="handleInit" :disabled="initializing">
            {{ initializing ? t("common.loading") : t("onboarding.init.initButton") }}
          </n-button>
        </template>
      </div>
      <template #footer>
        <div style="display: flex; justify-content: space-between;">
          <n-button quaternary @click="router.push('/onboarding/project')">{{ t("common.back") }}</n-button>
          <n-button :disabled="!done" @click="router.push('/onboarding/ssh')">{{ t("common.next") }}</n-button>
        </div>
      </template>
    </n-card>
  </n-spin>
</template>
