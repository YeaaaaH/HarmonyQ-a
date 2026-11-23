export type ItemStatus = "open" | "done";

export interface Item {
  id: string;
  title: string;
  status: ItemStatus;
  createdAt: number;
  updatedAt: number;
}
