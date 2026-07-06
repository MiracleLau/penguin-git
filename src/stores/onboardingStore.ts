import { defineStore } from "pinia";
import { ref } from "vue";

export const useOnboardingStore = defineStore("onboarding", () => {
  const gitDetected = ref<boolean | null>(null);
  const gitVersion = ref<string | null>(null);
  const projectPath = ref<string | null>(null);
  const projectName = ref<string | null>(null);
  const isGitRepo = ref<boolean | null>(null);
  const initialized = ref(false);
  const gitignoreContent = ref<string | null>(null);
  const sshKeyExists = ref<boolean | null>(null);
  const sshPublicKey = ref<string | null>(null);
  const platform = ref<string | null>(null);
  const remoteUrl = ref<string | null>(null);
  const keyAdded = ref(false);
  const connectionTested = ref(false);
  const connectionSuccess = ref<boolean | null>(null);

  function setGit(detected: boolean, version: string | null) {
    gitDetected.value = detected;
    gitVersion.value = version;
  }

  function setProject(path: string, name: string) {
    projectPath.value = path;
    projectName.value = name;
  }

  function setInit(success: boolean, gitignore: string | null) {
    initialized.value = success;
    gitignoreContent.value = gitignore;
  }

  function setSshKey(exists: boolean, publicKey: string | null) {
    sshKeyExists.value = exists;
    sshPublicKey.value = publicKey;
  }

  function setRemote(plat: string, url: string) {
    platform.value = plat;
    remoteUrl.value = url;
  }

  function setKeyAdded(v: boolean) {
    keyAdded.value = v;
  }

  function setConnection(success: boolean | null) {
    connectionTested.value = true;
    connectionSuccess.value = success;
  }

  function reset() {
    gitDetected.value = null;
    gitVersion.value = null;
    projectPath.value = null;
    projectName.value = null;
    isGitRepo.value = null;
    initialized.value = false;
    gitignoreContent.value = null;
    sshKeyExists.value = null;
    sshPublicKey.value = null;
    platform.value = null;
    remoteUrl.value = null;
    keyAdded.value = false;
    connectionTested.value = false;
    connectionSuccess.value = null;
  }

  return {
    gitDetected, gitVersion, projectPath, projectName, isGitRepo,
    initialized, gitignoreContent, sshKeyExists, sshPublicKey,
    platform, remoteUrl, keyAdded, connectionTested, connectionSuccess,
    setGit, setProject, setInit, setSshKey, setRemote,
    setKeyAdded, setConnection, reset,
  };
});
