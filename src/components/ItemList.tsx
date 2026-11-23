import { Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { cn } from "@/lib/utils";
import type { Item } from "@/types/item";

interface Props {
  items: Item[];
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

export function ItemList({ items, onToggle, onRemove }: Props) {
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
            <span className={cn("flex-1", done && "text-muted-foreground line-through")}>
              {item.title}
            </span>
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
