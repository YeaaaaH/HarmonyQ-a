import { useState } from "react";
import { ChevronDown, ChevronRight, Sun } from "lucide-react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList, ItemMeta } from "@/components/ItemList";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { projectColor } from "@/lib/projectColor";
import type { Item } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

interface Props {
  project: ProjectSummary;
  tasks: Item[];
  /** Today's local date, `YYYY-MM-DD`, once known. */
  today: string | null;
  onAdd: (title: string) => Promise<boolean>;
  onDoToday: (item: Item) => void;
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

export function ProjectView({ project, tasks, today, onAdd, onDoToday, ...listProps }: Props) {
  const [showDone, setShowDone] = useState(false);
  const nextStep = project.nextStep;
  const open = tasks.filter((t) => t.status === "open" && t.id !== nextStep?.id);
  const done = tasks.filter((t) => t.status === "done");

  return (
    <>
      <header className="flex flex-col gap-1">
        <h1 className="flex items-center gap-3 text-2xl font-semibold">
          <span
            className="size-3.5 shrink-0 rounded-full"
            style={{ backgroundColor: projectColor(project.hue) }}
          />
          <span className="min-w-0 truncate">{project.name}</span>
        </h1>
        {project.outcome && (
          <p className="text-sm text-muted-foreground">Done when: {project.outcome}</p>
        )}
      </header>

      <NextStepCard
        step={nextStep}
        hue={project.hue}
        today={today}
        onDoToday={onDoToday}
        onOpen={listProps.onOpen}
        onToggle={listProps.onToggle}
      />

      <AddItemForm onAdd={onAdd} placeholder="Add a step" />

      {open.length > 0 && (
        <section className="flex flex-col gap-2">
          <h2 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            Then
          </h2>
          <ItemList items={open} {...listProps} />
        </section>
      )}

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

function NextStepCard({
  step,
  hue,
  today,
  onDoToday,
  onOpen,
  onToggle,
}: {
  step: Item | null;
  hue: number;
  today: string | null;
  onDoToday: (item: Item) => void;
  onOpen: (item: Item) => void;
  onToggle: (item: Item) => void;
}) {
  return (
    <section
      className="flex flex-col gap-2 rounded-xl border border-l-4 bg-card p-4 shadow-sm"
      style={{ borderLeftColor: projectColor(hue) }}
    >
      <h2 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
        Next step
      </h2>
      {step ? (
        <div className="flex items-center gap-3">
          <Checkbox checked={false} onCheckedChange={() => onToggle(step)} />
          <button
            onClick={() => onOpen(step)}
            className="flex min-w-0 flex-1 flex-col items-start gap-0.5 text-left"
          >
            <span className="w-full truncate text-lg font-medium">{step.title}</span>
            <ItemMeta
              item={step}
              overdue={today !== null && step.dueDate !== null && step.dueDate < today}
            />
          </button>
          {today && step.dueDate !== today && (
            <Button variant="secondary" size="sm" onClick={() => onDoToday(step)}>
              <Sun />
              Do today
            </Button>
          )}
        </div>
      ) : (
        <div>
          <p className="text-lg font-medium">Needs a next step</p>
          <p className="text-sm text-muted-foreground">
            What is the next small thing to do here? Add it below.
          </p>
        </div>
      )}
    </section>
  );
}
