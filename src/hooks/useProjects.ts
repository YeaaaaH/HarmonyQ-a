import { useCallback, useEffect, useRef, useState } from "react";
import { useAction } from "@/hooks/useAction";
import * as api from "@/lib/api/projects";
import type { Item } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

/** Re-reads the summaries whenever `items` change: next steps and counts depend on them. */
export function useProjects(items: Item[]) {
  const [projects, setProjects] = useState<ProjectSummary[]>([]);
  const { error, setError, run } = useAction();
  const latest = useRef(0);

  const refresh = useCallback(async () => {
    // Refreshes can overlap; only the newest answer may land.
    const request = ++latest.current;
    try {
      const list = await api.listProjects();
      if (request !== latest.current) return;
      setProjects(list);
      setError(null);
    } catch (e) {
      if (request === latest.current) setError(String(e));
    }
  }, [setError]);

  useEffect(() => {
    refresh();
  }, [items, refresh]);

  const create = (name: string) =>
    run(async () => {
      const project = await api.createProject(name);
      await refresh();
      return project;
    });

  const turnTaskIntoProject = (itemId: string) =>
    run(async () => {
      const result = await api.turnTaskIntoProject(itemId);
      await refresh();
      return result;
    });

  return { projects, error, create, turnTaskIntoProject };
}

export interface ProjectTasks {
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
