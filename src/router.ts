import { createRouter, createWebHashHistory } from "vue-router";
import OnboardingLayout from "./views/Onboarding/OnboardingLayout.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: () => {
        const completed = localStorage.getItem("penguin-onboarding") === "true";
        return completed ? "/select" : "/onboarding";
      },
    },
    {
      path: "/onboarding",
      component: OnboardingLayout,
      children: [
        { path: "", name: "welcome", component: () => import("./views/Onboarding/WelcomeStep.vue") },
        { path: "git-check", component: () => import("./views/Onboarding/GitCheckStep.vue") },
        { path: "ssh", component: () => import("./views/Onboarding/SshKeyStep.vue") },
        { path: "done", component: () => import("./views/Onboarding/DoneStep.vue") },
      ],
    },
    {
      path: "/dashboard",
      component: () => import("./views/Dashboard/DashboardLayout.vue"),
      children: [
        { path: "", redirect: "/dashboard/changes" },
        { path: "changes", component: () => import("./views/Dashboard/ChangesView.vue") },
        { path: "history", component: () => import("./views/Dashboard/HistoryView.vue") },
        { path: "settings", component: () => import("./views/Dashboard/SettingsView.vue") },
      ],
    },
    { path: "/select", component: () => import("./views/ProjectSelect.vue") },
    { path: "/settings", component: () => import("./views/GlobalSettings.vue") },
  ],
});

export default router;
