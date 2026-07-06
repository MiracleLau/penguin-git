import { defineStore } from "pinia";
import { ref } from "vue";

export const useAppStore = defineStore("app", () => {
  const language = ref<"zh-CN" | "en-US">(
    (localStorage.getItem("penguin-lang") as "zh-CN" | "en-US") || "zh-CN"
  );
  const theme = ref<"light" | "dark" | "system">(
    (localStorage.getItem("penguin-theme") as "light" | "dark" | "system") || "light"
  );
  const onboardingCompleted = ref(localStorage.getItem("penguin-onboarding") === "true");

  function setLanguage(lang: "zh-CN" | "en-US") {
    localStorage.setItem("penguin-lang", lang);
    language.value = lang;
  }

  function setTheme(t: "light" | "dark" | "system") {
    localStorage.setItem("penguin-theme", t);
    theme.value = t;
  }

  function completeOnboarding() {
    localStorage.setItem("penguin-onboarding", "true");
    onboardingCompleted.value = true;
  }

  return { language, theme, onboardingCompleted, setLanguage, setTheme, completeOnboarding };
});
