<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { useOnboardingStore } from "../../stores/onboardingStore";
import { NCard, NButton, NInput, NRadio, NRadioGroup, NSpace } from "naive-ui";

const PLATFORMS = [
  { id: "github", label: "GitHub", host: "github.com" },
  { id: "codeberg", label: "Codeberg", host: "codeberg.org" },
  { id: "custom", label: "自定义", host: "" },
];

const { t } = useI18n();
const router = useRouter();
const { setRemote } = useOnboardingStore();
const platform = ref("github");
const remoteUrl = ref("");

function handleNext() {
  setRemote(platform.value, remoteUrl.value);
  router.push("/onboarding/add-key");
}
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header><h2 style="font-size: 16px; font-weight: 600;">{{ t("onboarding.remote.title") }}</h2></template>
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <div>
        <p style="font-size: 14px; font-weight: 500;">{{ t("onboarding.remote.platform") }}</p>
        <n-radio-group v-model:value="platform" style="margin-top: 8px;">
          <n-space>
            <n-radio v-for="p in PLATFORMS" :key="p.id" :value="p.id" :label="p.label" />
          </n-space>
        </n-radio-group>
      </div>
      <n-input v-model:value="remoteUrl" placeholder="git@github.com:username/repo.git" />
    </div>
    <template #footer>
      <div style="display: flex; justify-content: space-between;">
        <n-button quaternary @click="router.push('/onboarding/ssh')">{{ t("common.back") }}</n-button>
        <div style="display: flex; gap: 8px;">
          <n-button quaternary @click="router.push('/onboarding/done')">{{ t("common.skipStep") }}</n-button>
          <n-button type="primary" :disabled="!remoteUrl" @click="handleNext">{{ t("common.next") }}</n-button>
        </div>
      </div>
    </template>
  </n-card>
</template>
