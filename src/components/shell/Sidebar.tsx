import { cn } from "@/lib/utils";
import { VIEWS, type View } from "./views";

interface Props {
  active: View;
  counts: Partial<Record<View, number>>;
  onSelect: (view: View) => void;
}

export function Sidebar({ active, counts, onSelect }: Props) {
  return (
    <aside className="flex w-56 shrink-0 flex-col gap-4 border-r bg-sidebar p-3 text-sidebar-foreground">
      <div className="px-2 pt-1 text-lg font-semibold">HarmonyQ'a</div>
      <nav className="flex flex-col gap-1">
        {VIEWS.map(({ id, label, icon: Icon }) => (
          <button
            key={id}
            onClick={() => onSelect(id)}
            className={cn(
              "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors hover:bg-sidebar-accent",
              active === id && "bg-sidebar-accent font-medium text-sidebar-accent-foreground",
            )}
          >
            <Icon className="size-4" />
            <span className="flex-1 text-left">{label}</span>
            {counts[id] ? (
              <span className="text-xs text-muted-foreground">{counts[id]}</span>
            ) : null}
          </button>
        ))}
      </nav>
    </aside>
  );
}
