import type { Item } from "@/types/item";

export type ProjectStatus = "active" | "paused" | "done";

export interface Project {
  id: string;
  name: string;
  /** "Done when ...", may be empty. */
  outcome: string;
  status: ProjectStatus;
  resumeNote: string;
  /** 0-359, fixed at creation. */
  hue: number;
  createdAt: number;
  updatedAt: number;
  completedAt: number | null;
}

export interface ProjectSummary extends Project {
  /** First open task with a date or high priority; null when none has one. With openCount 0 the project needs a next step. */
  nextStep: Item | null;
  openCount: number;
  doneCount: number;
  lastActivityAt: number;
}
