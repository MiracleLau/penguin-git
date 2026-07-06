import { defineStore } from "pinia";
import { ref } from "vue";

export const useSettingsStore = defineStore("settings", () => {
  const gitImpl = ref<string | null>(null);
  const gitPath = ref<string | null>(null);
  const gitName = ref<string | null>(null);
  const gitEmail = ref<string | null>(null);
  const loaded = ref(false);

  function loadSettings(s: { gitImpl?: string | null; gitPath?: string | null; gitName?: string | null; gitEmail?: string | null }) {
    if (s.gitImpl !== undefined) gitImpl.value = s.gitImpl;
    if (s.gitPath !== undefined) gitPath.value = s.gitPath;
    if (s.gitName !== undefined) gitName.value = s.gitName;
    if (s.gitEmail !== undefined) gitEmail.value = s.gitEmail;
    loaded.value = true;
  }

  function updateSettings(partial: Partial<{ gitImpl: string | null; gitPath: string | null; gitName: string | null; gitEmail: string | null }>) {
    if (partial.gitImpl !== undefined) gitImpl.value = partial.gitImpl;
    if (partial.gitPath !== undefined) gitPath.value = partial.gitPath;
    if (partial.gitName !== undefined) gitName.value = partial.gitName;
    if (partial.gitEmail !== undefined) gitEmail.value = partial.gitEmail;
  }

  return { gitImpl, gitPath, gitName, gitEmail, loaded, loadSettings, updateSettings };
});
