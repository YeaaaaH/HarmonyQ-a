import { ListTodo, Sun, type LucideIcon } from "lucide-react";

export type View = "today" | "all";

export const VIEWS: { id: View; label: string; icon: LucideIcon }[] = [
  { id: "today", label: "Today", icon: Sun },
  { id: "all", label: "All tasks", icon: ListTodo },
];
