import { invoke } from "@tauri-apps/api/core";
import type { Item } from "@/types/item";
import type { Project, ProjectSummary } from "@/types/project";

export const listProjects = () => invoke<ProjectSummary[]>("list_projects");

export const listProjectTasks = (id: string) => invoke<Item[]>("list_project_tasks", { id });

export const turnTaskIntoProject = (itemId: string) =>
  invoke<Project>("turn_task_into_project", { itemId });

export const createProject = (name: string, outcome = "") =>
  invoke<Project>("create_project", { request: { name, outcome } });
