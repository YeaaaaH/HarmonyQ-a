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
