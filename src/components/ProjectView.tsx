import { useState } from "react";
import { Check, ChevronDown, ChevronRight, Sun, Trash2 } from "lucide-react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList, ItemMeta } from "@/components/ItemList";
import { Button } from "@/components/ui/button";
import { projectColor } from "@/lib/projectColor";
import { cn } from "@/lib/utils";
import type { Item } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

interface Props {
  project: ProjectSummary;
  /** Open tasks in next-step order, then done ones (backend order). */
  tasks: Item[];
  tasksError: string | null;
  tasksLoaded: boolean;
  /** Today's local date, `YYYY-MM-DD`, once known. */
  today: string | null;
  onAdd: (title: string) => Promise<boolean>;
  onDoToday: (item: Item) => void;
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

export function ProjectView({
  project,
  tasks,
  tasksError,
  tasksLoaded,
  today,
  onAdd,
  onDoToday,
  ...listProps
}: Props) {
  const [showDone, setShowDone] = useState(false);
  const open = tasks.filter((t) => t.status === "open");
  const done = tasks.filter((t) => t.status === "done");
  const color = projectColor(project.hue);

  return (
    <>
      <header className="flex flex-col gap-1">
        <h1 className="flex items-center gap-3 text-2xl font-semibold">
          <span className="size-3.5 shrink-0 rounded-full" style={{ backgroundColor: color }} />
          <span className="min-w-0 truncate">{project.name}</span>
        </h1>
        {project.outcome && (
          <p className="text-sm text-muted-foreground">Done when: {project.outcome}</p>
        )}
      </header>

      <AddItemForm onAdd={onAdd} placeholder="Add a step" />

      {tasksError && <p className="text-sm text-destructive">{tasksError}</p>}

      <ol className="flex flex-col gap-3 pt-1">
        {tasksLoaded && !tasksError && open.length === 0 && <EmptyStep />}
        {open.map((step, index) => (
          <Step
            key={step.id}
            step={step}
            next={index === 0 && project.nextStep?.id === step.id}
            color={color}
            today={today}
            onDoToday={onDoToday}
            {...listProps}
          />
        ))}
      </ol>

      {done.length > 0 && (
        <section className="flex flex-col gap-2">
          <button
            onClick={() => setShowDone((v) => !v)}
            className="flex items-center gap-1 text-xs font-medium tracking-wide text-muted-foreground uppercase hover:text-foreground [&_svg]:size-3.5"
          >
            {showDone ? <ChevronDown /> : <ChevronRight />}
            Done ({done.length})
          </button>
          {showDone && <ItemList items={done} {...listProps} />}
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
}: {
  step: Item;
  next: boolean;
  color: string;
  today: string | null;
  onDoToday: (item: Item) => void;
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
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
