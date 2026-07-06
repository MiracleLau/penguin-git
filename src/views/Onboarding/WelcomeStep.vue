<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { NCard, NButton } from "naive-ui";

const { t } = useI18n();
const router = useRouter();
</script>

<template>
  <n-card style="max-width: 400px; width: 100%; margin: 0 16px;">
    <template #header>
      <div style="text-align: center; padding: 20px 0 12px;">
        <div style="font-size: 32px; line-height: 1;">🐧</div>
        <h1 style="font-size: 22px; font-weight: 700; margin-top: 8px;">{{ t("onboarding.welcome.title") }}</h1>
        <p style="margin-top: 6px; opacity: 0.6;">{{ t("onboarding.welcome.subtitle") }}</p>
      </div>
    </template>
    <div style="text-align: center; padding-bottom: 12px;">
      <n-button size="large" type="primary" @click="router.push('/onboarding/git-check')" style="margin-bottom: 8px;">
        {{ t("onboarding.welcome.start") }}
      </n-button>
      <div>
        <n-button size="small" quaternary @click="router.push('/onboarding/done')">
          {{ t("common.skipOnboarding") }}
        </n-button>
      </div>
    </div>
  </n-card>
</template>
