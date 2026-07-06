<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { NCard, NButton, NInput, NModal, NTag, NButtonGroup, NIcon } from "naive-ui";
import {
  getSettings, setSettings, getGitGlobalConfig, setGitGlobalConfig, detectGitPath,
  getPublicKey, copyPublicKey, generateSshKey,
  listCredentials, saveCredential, removeCredential,
} from "../lib/tauri";
import { useSettingsStore } from "../stores/settingsStore";
import { useAppStore } from "../stores/appStore";
import { showSuccess, showDanger } from "../lib/notify";

const { t, i18n } = useI18n();
const router = useRouter();
const { language, setLanguage, theme, setTheme } = useAppStore();
const { loadSettings, updateSettings } = useSettingsStore();
const pubKey = ref<string | null>(null);
const copied = ref(false);
const editName = ref("");
const editEmail = ref("");
const creds = ref<{ url: string; username: string }[]>([]);
const newCredUrl = ref("");
const newCredUser = ref("");
const newCredPass = ref("");
const gitBackend = ref("gix");
const customGitPath = ref("");
const showSshModal = ref(false);

onMounted(async () => {
  const s = await getSettings();
  loadSettings(s);
  gitBackend.value = s.gitImpl || "gix";
  customGitPath.value = s.gitPath || "";
  editName.value = s.gitName || "";
  editEmail.value = s.gitEmail || "";

  getPublicKey().then((v) => { pubKey.value = v; }).catch(() => { pubKey.value = null; });
  listCredentials().then((v) => { creds.value = v; }).catch(() => {});
  getGitGlobalConfig("user.name").then((v) => { if (v && !editName.value) editName.value = v; }).catch(() => {});
  getGitGlobalConfig("user.email").then((v) => { if (v && !editEmail.value) editEmail.value = v; }).catch(() => {});
});

async function handleSaveIdentity() {
  try {
    await setGitGlobalConfig("user.name", editName.value);
    await setGitGlobalConfig("user.email", editEmail.value);
    updateSettings({ gitName: editName.value, gitEmail: editEmail.value });
    showSuccess("用户身份已保存");
  } catch (e) { showDanger("保存失败", String(e)); }
}

async function handleSaveBackend() {
  try {
    await setSettings({ gitImpl: gitBackend.value, gitPath: customGitPath.value || null, gitName: editName.value, gitEmail: editEmail.value });
    showSuccess("设置已保存");
  } catch (e) { showDanger("保存失败", String(e)); }
}

async function handleDetectPath() {
  const path = await detectGitPath();
  if (path) { customGitPath.value = path; showSuccess("检测到: " + path); }
  else { showDanger("未找到系统 git"); }
}

async function handleCopyKey() {
  try {
    await copyPublicKey();
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 2000);
    showSuccess("已复制");
  } catch { showDanger("复制失败"); }
}

async function handleRegenKey() {
  try {
    await generateSshKey("penguin-git");
    const newKey = await getPublicKey();
    pubKey.value = newKey;
    showSshModal.value = false;
    showSuccess("SSH 密钥已重新生成，请将新公钥添加到各平台");
  } catch (e) { showDanger("生成失败", String(e)); }
}

async function handleAddCredential() {
  if (!newCredUrl.value || !newCredUser.value || !newCredPass.value) return;
  try {
    await saveCredential(newCredUrl.value, newCredUser.value, newCredPass.value);
    creds.value = await listCredentials();
    newCredUrl.value = ""; newCredUser.value = ""; newCredPass.value = "";
    showSuccess("凭据已保存");
  } catch (e) { showDanger("保存失败", String(e)); }
}

async function handleRemoveCred(url: string) {
  try { await removeCredential(url); creds.value = await listCredentials(); showSuccess("凭据已删除"); }
  catch (e) { showDanger("删除失败", String(e)); }
}
</script>

<template>
  <div style="min-height: 100vh; padding: 16px;">
    <div style="max-width: 720px; margin: 0 auto; display: flex; flex-direction: column; gap: 12px;">
      <div style="display: flex; align-items: center; gap: 6px;">
        <n-button circle size="small" quaternary @click="router.go(-1)">
          <template #icon><n-icon><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5"/><path d="M12 19l-7-7 7-7"/></svg></n-icon></template>
        </n-button>
        <h1 style="font-size: 18px; font-weight: 600;">全局设置</h1>
      </div>

      <!-- SSH Key -->
      <n-card size="small">
        <template #header><h3 style="font-size: 14px; font-weight: 500;">Git 身份 — SSH 密钥</h3></template>
        <template v-if="pubKey">
          <pre style="font-size: 12px; background: rgba(128,128,128,0.06); padding: 8px; border-radius: 12px; overflow-x: auto; white-space: pre-wrap; word-break: break-all; max-height: 64px; margin-bottom: 6px;">{{ pubKey }}</pre>
          <div style="display: flex; gap: 6px;">
            <n-button size="small" quaternary @click="handleCopyKey">{{ copied ? "已复制" : "复制公钥" }}</n-button>
            <n-button size="small" quaternary @click="showSshModal = true">重新生成密钥</n-button>
          </div>
        </template>
        <template v-else>
          <p style="font-size: 14px; opacity: 0.5; margin-bottom: 6px;">未找到 SSH 密钥</p>
          <n-button size="small" @click="showSshModal = true">生成密钥</n-button>
        </template>
      </n-card>

      <!-- HTTPS Credentials -->
      <n-card size="small">
        <template #header><h3 style="font-size: 13px; font-weight: 500;">Git 身份 — HTTPS 凭据</h3></template>
        <div v-if="creds.length === 0" style="font-size: 14px; opacity: 0.5; margin-bottom: 6px;">暂无 HTTPS 凭据</div>
        <div v-for="c in creds" :key="c.url" style="display: flex; align-items: center; justify-content: space-between; font-size: 14px; padding: 4px 0;">
          <span>{{ c.url }} ({{ c.username }})</span>
          <n-button size="small" quaternary @click="handleRemoveCred(c.url)">删除</n-button>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 6px; margin-top: 6px;">
          <n-input v-model:value="newCredUrl" placeholder="github.com" />
          <n-input v-model:value="newCredUser" placeholder="用户名" />
          <n-input v-model:value="newCredPass" placeholder="密码/Token" type="password" />
        </div>
        <n-button size="small" type="primary" style="margin-top: 6px;" @click="handleAddCredential" :disabled="!newCredUrl || !newCredUser || !newCredPass">添加凭据</n-button>
      </n-card>

      <!-- Identity -->
      <n-card size="small">
        <template #header><h3 style="font-size: 13px; font-weight: 500;">用户姓名和邮箱</h3></template>
        <div style="display: flex; flex-direction: column; gap: 6px;">
          <n-input v-model:value="editName" placeholder="张三" />
          <n-input v-model:value="editEmail" placeholder="zhangsan@example.com" />
          <n-button type="primary" @click="handleSaveIdentity">保存（写入 git config --global）</n-button>
        </div>
      </n-card>

      <!-- Git Backend -->
      <n-card size="small">
        <template #header><h3 style="font-size: 13px; font-weight: 500;">使用方式</h3></template>
        <div style="display: flex; flex-direction: column; gap: 6px;">
          <div style="display: flex; gap: 12px;">
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer; font-size: 14px;">
              <input type="radio" name="backend" :checked="gitBackend === 'gix'" @change="gitBackend = 'gix'" />
              内置 git（gix，默认）
            </label>
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer; font-size: 14px;">
              <input type="radio" name="backend" :checked="gitBackend === 'cli'" @change="gitBackend = 'cli'" />
              系统 git CLI
            </label>
          </div>
          <div v-if="gitBackend === 'cli'" style="display: flex; gap: 6px;">
            <n-input v-model:value="customGitPath" placeholder="git 或 /usr/local/bin/git" />
            <n-button size="small" quaternary @click="handleDetectPath">自动检测</n-button>
          </div>
          <n-button type="primary" @click="handleSaveBackend">保存使用方式</n-button>
        </div>
      </n-card>

      <!-- Language -->
      <n-card size="small">
        <template #header><h3 style="font-size: 13px; font-weight: 500;">语言</h3></template>
        <n-button-group>
          <n-button :type="language === 'zh-CN' ? 'primary' : 'default'" @click="setLanguage('zh-CN'); i18n.locale = 'zh-CN'">中文</n-button>
          <n-button :type="language === 'en-US' ? 'primary' : 'default'" @click="setLanguage('en-US'); i18n.locale = 'en-US'">English</n-button>
        </n-button-group>
      </n-card>

      <!-- Theme -->
      <n-card size="small">
        <template #header><h3 style="font-size: 13px; font-weight: 500;">主题</h3></template>
        <n-button-group>
          <n-button :type="theme === 'light' ? 'primary' : 'default'" @click="setTheme('light')">浅色</n-button>
          <n-button :type="theme === 'dark' ? 'primary' : 'default'" @click="setTheme('dark')">深色</n-button>
          <n-button :type="theme === 'system' ? 'primary' : 'default'" @click="setTheme('system')">跟随系统</n-button>
        </n-button-group>
      </n-card>
    </div>

    <!-- SSH confirmation modal -->
    <n-modal v-model:show="showSshModal" preset="card" title="确认重新生成 SSH 密钥" style="max-width: 480px;" closable>
      <p style="font-size: 14px; opacity: 0.7; margin-bottom: 16px;">
        这将覆盖现有密钥。已配置到 GitHub、Codeberg 等平台的旧公钥将立即失效，需要重新添加新公钥。
      </p>
      <div style="display: flex; justify-content: flex-end; gap: 6px;">
        <n-button quaternary @click="showSshModal = false">取消</n-button>
        <n-button type="primary" @click="handleRegenKey">确认重新生成</n-button>
      </div>
    </n-modal>
  </div>
</template>
