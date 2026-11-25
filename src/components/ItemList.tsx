import { CalendarDays, Clock, Flag, StickyNote, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { ProjectDot } from "@/components/ProjectDot";
import { cn } from "@/lib/utils";
import type { Item } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

export interface ItemHandlers {
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

interface Props extends ItemHandlers {
  items: Item[];
  /** Marks the dates as overdue. */
  overdue?: boolean;
  /** When given, each task shows its project's color and name. */
  projects?: Map<string, ProjectSummary>;
}

export function ItemList({
  items,
  overdue = false,
  projects,
  onOpen,
  onToggle,
  onRemove,
}: Props) {
  if (items.length === 0) {
    return <p className="py-8 text-center text-muted-foreground">Nothing here yet.</p>;
  }

  return (
    <ul className="divide-y rounded-lg border bg-card">
      {items.map((item) => {
        const done = item.status === "done";
        return (
          <li key={item.id} className="group flex items-center gap-3 px-3 py-2">
            <Checkbox checked={done} onCheckedChange={() => onToggle(item)} />
            <button
              onClick={() => onOpen(item)}
              className="flex min-w-0 flex-1 flex-col items-start text-left"
            >
              <span
                className={cn("w-full truncate", done && "text-muted-foreground line-through")}
              >
                {item.title}
              </span>
              <ItemMeta
                item={item}
                overdue={overdue}
                project={item.projectId ? projects?.get(item.projectId) : undefined}
              />
            </button>
            <Button
              variant="ghost"
              size="icon"
              aria-label="Delete"
              onClick={() => onRemove(item.id)}
              className="text-muted-foreground opacity-0 group-hover:opacity-100 hover:text-destructive"
            >
              <Trash2 />
            </Button>
          </li>
        );
      })}
    </ul>
  );
}

export function ItemMeta({
  item,
  overdue,
  project,
}: {
  item: Item;
  overdue: boolean;
  project?: ProjectSummary;
}) {
  const high = item.priority === "high";
  if (!item.dueDate && !item.note && !high && !project) return null;

  return (
    <span className="flex min-w-0 items-center gap-3 text-xs text-muted-foreground [&_svg]:size-3.5">
      {project && (
        <span className="flex min-w-0 items-center gap-1.5">
          <ProjectDot hue={project.hue} className="size-2" />
          <span className="truncate">{project.name}</span>
        </span>
      )}
      {item.dueDate && (
        <span className={cn("flex items-center gap-1", overdue && "text-destructive")}>
          <CalendarDays />
          {formatDate(item.dueDate)}
        </span>
      )}
      {item.dueTime && (
        <span className="flex items-center gap-1">
          <Clock />
          {item.dueTime}
        </span>
      )}
      {high && (
        <span className="flex items-center gap-1 text-warning">
          <Flag className="fill-current" />
          Important
        </span>
      )}
      {item.note && <StickyNote aria-label="Has a note" />}
    </span>
  );
}

function formatDate(date: string) {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year, month - 1, day).toLocaleDateString(undefined, {
    weekday: "short",
    day: "numeric",
    month: "short",
  });
}
