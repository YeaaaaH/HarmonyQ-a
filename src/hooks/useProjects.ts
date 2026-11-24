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

  const turnTaskIntoProject = async (itemId: string): Promise<Project | null> => {
    try {
      setError(null);
      return await api.turnTaskIntoProject(itemId);
    } catch (e) {
      setError(String(e));
      return null;
    }
  };

  return { projects, error, create, turnTaskIntoProject };
}

interface ProjectTasks {
  tasks: Item[];
  error: string | null;
  /** False until the first answer for this project arrives. */
  loaded: boolean;
}

/** Tasks of one project in the backend's order: open by next step, then done. */
export function useProjectTasks(projectId: string | null, items: Item[]): ProjectTasks {
  const [state, setState] = useState<ProjectTasks & { projectId: string | null }>({
    projectId: null,
    tasks: [],
    error: null,
    loaded: false,
  });

  useEffect(() => {
    if (!projectId) return;
    // Ignore a late answer for a project the user has already left.
    let current = true;
    api.listProjectTasks(projectId).then(
      (tasks) => current && setState({ projectId, tasks, error: null, loaded: true }),
      (e) => current && setState({ projectId, tasks: [], error: String(e), loaded: true }),
    );
    return () => {
      current = false;
    };
  }, [projectId, items]);

  // Keyed by project, so another project's steps never show under this one.
  if (state.projectId !== projectId) return { tasks: [], error: null, loaded: false };
  return state;
}
