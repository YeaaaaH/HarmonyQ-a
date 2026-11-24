import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/api/projects";
import type { Item } from "@/types/item";
import type { Project, ProjectSummary } from "@/types/project";

/** Re-reads the summaries whenever `items` change: next steps and counts depend on them. */
export function useProjects(items: Item[]) {
  const [projects, setProjects] = useState<ProjectSummary[]>([]);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(
    () => api.listProjects().then(setProjects, (e) => setError(String(e))),
    [],
  );

  useEffect(() => {
    refresh();
  }, [items, refresh]);

  const create = async (name: string): Promise<Project | null> => {
    try {
      setError(null);
      const project = await api.createProject(name);
      await refresh();
      return project;
    } catch (e) {
      setError(String(e));
      return null;
    }
  };

  return { projects, error, create };
}

/** Tasks of one project in the backend's order: open by next step, then done. */
export function useProjectTasks(projectId: string | null, items: Item[]) {
  const [tasks, setTasks] = useState<Item[]>([]);

  useEffect(() => {
    if (!projectId) {
      setTasks([]);
      return;
    }
    // Ignore a late answer for a project the user has already left.
    let current = true;
    api.listProjectTasks(projectId).then(
      (list) => current && setTasks(list),
      () => current && setTasks([]),
    );
    return () => {
      current = false;
    };
  }, [projectId, items]);

  return tasks;
}
