<script setup lang="ts">
import { computed, watch, onMounted } from "vue";
import { NConfigProvider, NMessageProvider, NDialogProvider, darkTheme } from "naive-ui";
import { useAppStore } from "./stores/appStore";
import { lightThemeOverrides, darkThemeOverrides } from "./theme/themes";
import MessageBridge from "./components/MessageBridge.vue";

const appStore = useAppStore();

function resolveTheme(): "light" | "dark" {
  if (appStore.theme === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return appStore.theme as "light" | "dark";
}

const naiveTheme = computed(() => (resolveTheme() === "dark" ? darkTheme : null));
const themeOverrides = computed(() =>
  resolveTheme() === "dark" ? darkThemeOverrides : lightThemeOverrides
);

watch(() => appStore.theme, () => {
  document.documentElement.classList.toggle("dark", resolveTheme() === "dark");
});

onMounted(() => {
  document.documentElement.classList.toggle("dark", resolveTheme() === "dark");
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (appStore.theme === "system") {
      document.documentElement.classList.toggle("dark", window.matchMedia("(prefers-color-scheme: dark)").matches);
    }
  });
});
</script>

<template>
  <NConfigProvider :theme="naiveTheme" :theme-overrides="themeOverrides">
    <NMessageProvider>
      <MessageBridge />
      <NDialogProvider>
        <router-view />
      </NDialogProvider>
    </NMessageProvider>
  </NConfigProvider>
</template>
