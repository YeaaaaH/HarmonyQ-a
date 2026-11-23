import { CalendarDays, Clock, Flag, StickyNote, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { cn } from "@/lib/utils";
import type { Item } from "@/types/item";

interface Props {
  items: Item[];
  /** Marks the dates as overdue. */
  overdue?: boolean;
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

export function ItemList({ items, overdue = false, onOpen, onToggle, onRemove }: Props) {
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
              <ItemMeta item={item} overdue={overdue} />
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

function ItemMeta({ item, overdue }: { item: Item; overdue: boolean }) {
  const high = item.priority === "high";
  if (!item.dueDate && !item.note && !high) return null;

  return (
    <span className="flex items-center gap-3 text-xs text-muted-foreground [&_svg]:size-3.5">
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
