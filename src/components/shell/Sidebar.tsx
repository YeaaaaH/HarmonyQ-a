import { useState, type KeyboardEvent } from "react";
import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { projectColor } from "@/lib/projectColor";
import { cn } from "@/lib/utils";
import type { ProjectSummary } from "@/types/project";
import { VIEWS, type View } from "./views";

interface Props {
  active: View | null;
  counts: Partial<Record<View, number>>;
  onSelect: (view: View) => void;
  projects: ProjectSummary[];
  activeProjectId: string | null;
  onSelectProject: (id: string) => void;
  onCreateProject: (name: string) => Promise<boolean>;
}

export function Sidebar({
  active,
  counts,
  onSelect,
  projects,
  activeProjectId,
  onSelectProject,
  onCreateProject,
}: Props) {
  const [creating, setCreating] = useState(false);
  const activeProjects = projects.filter((p) => p.status === "active");

  return (
    <aside className="flex w-56 shrink-0 flex-col gap-4 border-r bg-sidebar p-3 text-sidebar-foreground">
      <div className="px-2 pt-1 text-lg font-semibold">HarmonyQ'a</div>
      <nav className="flex flex-col gap-1">
        {VIEWS.map(({ id, label, icon: Icon }) => (
          <NavButton
            key={id}
            selected={active === id}
            count={counts[id]}
            onClick={() => onSelect(id)}
          >
            <Icon className="size-4" />
            <span className="flex-1 truncate text-left">{label}</span>
          </NavButton>
        ))}
      </nav>

      <section className="flex min-h-0 flex-1 flex-col gap-1">
        <div className="flex items-center justify-between px-2">
          <h2 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            Projects
          </h2>
          <Button
            variant="ghost"
            size="icon"
            aria-label="New project"
            onClick={() => setCreating(true)}
            className="size-6 text-muted-foreground"
          >
            <Plus />
          </Button>
        </div>
        <div className="flex min-h-0 flex-col gap-1 overflow-y-auto">
          {creating && (
            <NewProjectInput
              onCreate={async (name) => {
                if (await onCreateProject(name)) setCreating(false);
              }}
              onCancel={() => setCreating(false)}
            />
          )}
          {activeProjects.map((project) => (
            <NavButton
              key={project.id}
              selected={activeProjectId === project.id}
              count={project.openCount}
              onClick={() => onSelectProject(project.id)}
            >
              <span
                className="size-2.5 shrink-0 rounded-full"
                style={{ backgroundColor: projectColor(project.hue) }}
              />
              <span className="flex-1 truncate text-left">{project.name}</span>
            </NavButton>
          ))}
          {activeProjects.length === 0 && !creating && (
            <p className="px-2 text-xs text-muted-foreground">No projects yet.</p>
          )}
        </div>
      </section>
    </aside>
  );
}

function NavButton({
  selected,
  count,
  onClick,
  children,
}: {
  selected: boolean;
  count?: number;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm transition-colors hover:bg-sidebar-accent",
        selected && "bg-sidebar-accent font-medium text-sidebar-accent-foreground",
      )}
    >
      {children}
      {count ? <span className="text-xs text-muted-foreground">{count}</span> : null}
    </button>
  );
}

function NewProjectInput({
  onCreate,
  onCancel,
}: {
  onCreate: (name: string) => Promise<void>;
  onCancel: () => void;
}) {
  const [name, setName] = useState("");
  const [busy, setBusy] = useState(false);

  const handleKey = async (event: KeyboardEvent) => {
    if (event.key === "Escape") onCancel();
    if (event.key !== "Enter" || !name.trim() || busy) return;
    setBusy(true);
    await onCreate(name);
    setBusy(false);
  };

  return (
    <Input
      autoFocus
      value={name}
      onChange={(e) => setName(e.target.value)}
      onKeyDown={handleKey}
      onBlur={() => !name.trim() && onCancel()}
      placeholder="Project name"
      className="h-8 text-sm"
    />
  );
}
