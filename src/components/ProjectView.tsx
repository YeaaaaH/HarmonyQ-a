import { useState } from "react";
import { Check, ChevronDown, ChevronRight, Sun, Trash2 } from "lucide-react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList, ItemMeta, type ItemHandlers } from "@/components/ItemList";
import { ProjectDot } from "@/components/ProjectDot";
import { sectionHeading } from "@/components/Section";
import { Button } from "@/components/ui/button";
import type { ProjectTasks } from "@/hooks/useProjects";
import { projectColor } from "@/lib/projectColor";
import { cn } from "@/lib/utils";
import type { Item } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

interface Props extends ItemHandlers {
  project: ProjectSummary;
  tasks: ProjectTasks;
  /** Today's local date, `YYYY-MM-DD`, once known. */
  today: string | null;
  onAdd: (title: string) => Promise<boolean>;
  onDoToday: (item: Item) => void;
}

export function ProjectView({ project, tasks, today, onAdd, onDoToday, ...handlers }: Props) {
  const [showDone, setShowDone] = useState(false);
  const open = tasks.tasks.filter((t) => t.status === "open");
  const done = tasks.tasks.filter((t) => t.status === "done");
  const color = projectColor(project.hue);

  return (
    <>
      <header className="flex flex-col gap-1">
        <h1 className="flex items-center gap-3 text-2xl font-semibold">
          <ProjectDot hue={project.hue} className="size-3.5" />
          <span className="min-w-0 truncate">{project.name}</span>
        </h1>
        {project.outcome && (
          <p className="text-sm text-muted-foreground">Done when: {project.outcome}</p>
        )}
      </header>

      <AddItemForm onAdd={onAdd} placeholder="Add a step" />

      {tasks.error && <p className="text-sm text-destructive">{tasks.error}</p>}

      <ol className="flex flex-col gap-3 pt-1">
        {tasks.loaded && !tasks.error && open.length === 0 && <EmptyStep />}
        {open.map((step) => (
          <Step
            key={step.id}
            step={step}
            next={project.nextStep?.id === step.id}
            color={color}
            today={today}
            onDoToday={onDoToday}
            {...handlers}
          />
        ))}
      </ol>

      {done.length > 0 && (
        <section className="flex flex-col gap-2">
          <button
            onClick={() => setShowDone((v) => !v)}
            className={cn(sectionHeading, "flex items-center gap-1 hover:text-foreground [&_svg]:size-3.5")}
          >
            {showDone ? <ChevronDown /> : <ChevronRight />}
            Done ({done.length})
          </button>
          {showDone && <ItemList items={done} {...handlers} />}
        </section>
      )}
    </>
  );
}

function Step({
  step,
  next,
  color,
  today,
  onDoToday,
  onOpen,
  onToggle,
  onRemove,
}: ItemHandlers & {
  step: Item;
  next: boolean;
  color: string;
  today: string | null;
  onDoToday: (item: Item) => void;
}) {
  const overdue = today !== null && step.dueDate !== null && step.dueDate < today;

  return (
    <li className="group flex gap-3">
      <div className="flex w-5 shrink-0 justify-center">
        <button
          aria-label="Mark done"
          onClick={() => onToggle(step)}
          className={cn(
            "group/marker flex shrink-0 items-center justify-center rounded-full border-2 transition-colors",
            next ? "mt-[5px] size-3.5" : "mt-1 size-3 opacity-60 hover:opacity-100",
          )}
          style={{ borderColor: color, backgroundColor: next ? color : undefined }}
        >
          <Check
            strokeWidth={4}
            className={cn(
              "size-2 opacity-0 group-hover/marker:opacity-100",
              next ? "text-background" : "text-foreground",
            )}
          />
        </button>
      </div>

      <div className="flex min-w-0 flex-1 items-start gap-2">
        <button
          onClick={() => onOpen(step)}
          className="flex min-w-0 flex-1 flex-col items-start gap-0.5 text-left"
        >
          <span className={cn("w-full truncate", next ? "text-base font-medium" : "text-sm")}>
            {step.title}
          </span>
          <ItemMeta item={step} overdue={overdue} />
        </button>
        {next && today && step.dueDate !== today && (
          <Button
            variant="ghost"
            size="sm"
            onClick={() => onDoToday(step)}
            className="text-muted-foreground"
          >
            <Sun />
            Do today
          </Button>
        )}
        <Button
          variant="ghost"
          size="icon"
          aria-label="Delete"
          onClick={() => onRemove(step.id)}
          className="size-7 text-muted-foreground opacity-0 group-hover:opacity-100 hover:text-destructive"
        >
          <Trash2 />
        </Button>
      </div>
    </li>
  );
}

function EmptyStep() {
  return (
    <li className="flex gap-3">
      <div className="flex w-5 shrink-0 justify-center">
        <span className="mt-[5px] size-3.5 rounded-full border-2 border-dashed border-muted-foreground/50" />
      </div>
      <div>
        <p className="text-base font-medium">Needs a next step</p>
        <p className="text-sm text-muted-foreground">
          What is the next small thing to do here? Add it above.
        </p>
      </div>
    </li>
  );
}
