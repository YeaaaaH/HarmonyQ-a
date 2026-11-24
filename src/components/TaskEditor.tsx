import { useState, type FormEvent } from "react";
import { Flag, FolderPlus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet";
import { Textarea } from "@/components/ui/textarea";
import { projectColor } from "@/lib/projectColor";
import { cn } from "@/lib/utils";
import type { Item, ItemDetails } from "@/types/item";
import type { ProjectSummary } from "@/types/project";

// Radix Select cannot use "" as an item value.
const NO_PROJECT = "none";

interface Props {
  item: Item | null;
  error: string | null;
  projects: ProjectSummary[];
  onSave: (id: string, details: ItemDetails) => Promise<boolean>;
  onMove: (id: string, projectId: string | null) => Promise<boolean>;
  onTurnIntoProject: (item: Item) => void;
  onClose: () => void;
}

export function TaskEditor({ item, onClose, ...props }: Props) {
  return (
    <Sheet open={item !== null} onOpenChange={(open) => !open && onClose()}>
      <SheetContent>
        <SheetHeader>
          <SheetTitle>Edit task</SheetTitle>
          <SheetDescription className="sr-only">Date, time, note and priority</SheetDescription>
        </SheetHeader>
        {item && (
          // Keyed by id so the form state resets when another task is opened.
          <EditorForm key={item.id} item={item} onClose={onClose} {...props} />
        )}
      </SheetContent>
    </Sheet>
  );
}

function EditorForm({
  item,
  error,
  projects,
  onSave,
  onMove,
  onTurnIntoProject,
  onClose,
}: Props & { item: Item }) {
  const [details, setDetails] = useState<ItemDetails>({
    title: item.title,
    dueDate: item.dueDate,
    dueTime: item.dueTime,
    note: item.note,
    priority: item.priority,
  });
  const [projectId, setProjectId] = useState(item.projectId);
  const set = (patch: Partial<ItemDetails>) => setDetails((d) => ({ ...d, ...patch }));
  const high = details.priority === "high";
  // Paused or done projects are hidden, except the one the task is already in.
  const choices = projects.filter((p) => p.status === "active" || p.id === item.projectId);

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    if (!(await onSave(item.id, details))) return;
    if (projectId !== item.projectId && !(await onMove(item.id, projectId))) return;
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="flex flex-1 flex-col gap-4 overflow-y-auto px-4">
      <Field label="Title" htmlFor="title">
        <Input
          id="title"
          value={details.title}
          onChange={(e) => set({ title: e.target.value })}
        />
      </Field>

      <div className="grid grid-cols-2 gap-3">
        <Field label="Date" htmlFor="date">
          <Input
            id="date"
            type="date"
            value={details.dueDate ?? ""}
            onChange={(e) =>
              set({ dueDate: e.target.value || null, ...(!e.target.value && { dueTime: null }) })
            }
          />
        </Field>
        <Field label="Time" htmlFor="time">
          <Input
            id="time"
            type="time"
            disabled={!details.dueDate}
            value={details.dueTime ?? ""}
            onChange={(e) => set({ dueTime: e.target.value || null })}
          />
        </Field>
      </div>

      <Field label="Project">
        <div className="flex gap-2">
          <Select
            value={projectId ?? NO_PROJECT}
            onValueChange={(value) => setProjectId(value === NO_PROJECT ? null : value)}
          >
            <SelectTrigger className="min-w-0 flex-1">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={NO_PROJECT}>No project</SelectItem>
              {choices.map((project) => (
                <SelectItem key={project.id} value={project.id}>
                  <span
                    className="size-2.5 shrink-0 rounded-full"
                    style={{ backgroundColor: projectColor(project.hue) }}
                  />
                  {project.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {!item.projectId && (
            <Button type="button" variant="outline" onClick={() => onTurnIntoProject(item)}>
              <FolderPlus />
              Turn into project
            </Button>
          )}
        </div>
      </Field>

      <Field label="Priority">
        <Button
          type="button"
          variant="outline"
          aria-pressed={high}
          onClick={() => set({ priority: high ? "normal" : "high" })}
          className={cn("w-fit", high && "border-warning text-warning")}
        >
          <Flag className={cn(high && "fill-current")} />
          {high ? "Important" : "Normal"}
        </Button>
      </Field>

      <Field label="Note" htmlFor="note">
        <Textarea
          id="note"
          rows={6}
          value={details.note}
          onChange={(e) => set({ note: e.target.value })}
          placeholder="Details, links, anything to remember"
        />
      </Field>

      {error && <p className="text-sm text-destructive">{error}</p>}

      <SheetFooter className="mt-auto px-0">
        <Button type="submit">Save</Button>
        <Button type="button" variant="ghost" onClick={onClose}>
          Cancel
        </Button>
      </SheetFooter>
    </form>
  );
}

function Field({
  label,
  htmlFor,
  children,
}: {
  label: string;
  htmlFor?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={htmlFor}>{label}</Label>
      {children}
    </div>
  );
}
