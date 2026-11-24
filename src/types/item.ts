export type ItemStatus = "open" | "done";

export type Priority = "normal" | "high";

export interface Item {
  id: string;
  title: string;
  status: ItemStatus;
  /** Local calendar day, `YYYY-MM-DD`. */
  dueDate: string | null;
  /** Local time, `HH:MM`; only set together with `dueDate`. */
  dueTime: string | null;
  note: string;
  priority: Priority;
  projectId: string | null;
  /** Unix ms when ticked; null while open and for tasks done before projects existed. */
  completedAt: number | null;
  createdAt: number;
  updatedAt: number;
}

export interface ItemDetails {
  title: string;
  dueDate: string | null;
  dueTime: string | null;
  note: string;
  priority: Priority;
}

export interface TodayView {
  /** Local date the view was built for, `YYYY-MM-DD`. */
  date: string;
  overdue: Item[];
  today: Item[];
}
