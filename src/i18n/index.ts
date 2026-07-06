import { createI18n } from "vue-i18n";
import zhCN from "./zh-CN.json";
import enUS from "./en-US.json";

const savedLang =
  typeof window !== "undefined"
    ? localStorage.getItem("penguin-lang") || "zh-CN"
    : "zh-CN";

export default createI18n({
  legacy: false,
  locale: savedLang,
  fallbackLocale: "zh-CN",
  messages: {
    "zh-CN": zhCN,
    "en-US": enUS,
  },
});
