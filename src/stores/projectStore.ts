import { defineStore } from "pinia";
import { ref } from "vue";
import type { ProjectInfo } from "../lib/types";

export const useProjectStore = defineStore("project", () => {
  const currentProject = ref<ProjectInfo | null>(null);
  const recentProjects = ref<{ path: string; name: string; lastOpened: number }[]>([]);

  function setCurrentProject(project: ProjectInfo) {
    currentProject.value = project;
  }

  function setRecentProjects(projects: { path: string; name: string; lastOpened: number }[]) {
    recentProjects.value = projects;
  }

  function clearCurrentProject() {
    currentProject.value = null;
  }

  return { currentProject, recentProjects, setCurrentProject, setRecentProjects, clearCurrentProject };
});
