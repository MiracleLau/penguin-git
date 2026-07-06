<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { NProgress } from "naive-ui";
import { RouterView } from "vue-router";

const route = useRoute();
const STEPS = [
  "/onboarding", "/onboarding/git-check",
  "/onboarding/ssh", "/onboarding/done",
];
const currentStep = computed(() => {
  const idx = STEPS.indexOf(route.path);
  return idx >= 0 ? idx : 0;
});
const progress = computed(() => Math.round((currentStep.value / (STEPS.length - 1)) * 100));
</script>

<template>
  <div style="min-height: 100vh; display: flex; flex-direction: column;">
    <div style="width: 100%; max-width: 560px; margin: 0 auto; padding: 16px 16px 0;">
      <n-progress type="line" :percentage="progress" :indicator-placement="'inside'" />
    </div>
    <div style="flex: 1; display: flex; align-items: center; justify-content: center;">
      <router-view />
    </div>
  </div>
</template>
