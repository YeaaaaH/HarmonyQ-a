import { useMemo, useState } from "react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList, type ItemHandlers } from "@/components/ItemList";
import { ProjectView } from "@/components/ProjectView";
import { Section } from "@/components/Section";
import { TaskEditor } from "@/components/TaskEditor";
import { Sidebar } from "@/components/shell/Sidebar";
import { VIEWS, type Screen } from "@/components/shell/views";
import { useItems } from "@/hooks/useItems";
import { useProjects, useProjectTasks } from "@/hooks/useProjects";
import { useSystemTheme } from "@/hooks/useSystemTheme";
import { useToday } from "@/hooks/useToday";
import type { Item } from "@/types/item";

export default function App() {
  useSystemTheme();
  const [screen, setScreen] = useState<Screen>({ kind: "view", view: "today" });
  const projectId = screen.kind === "project" ? screen.id : null;
  const { items, error, clearError, replace, add, update, scheduleToday, toggle, remove } =
    useItems();
  const today = useToday(items);
  const {
    projects,
    error: projectsError,
    create: createProject,
    turnTaskIntoProject,
  } = useProjects(items);
  const projectsById = useMemo(() => new Map(projects.map((p) => [p.id, p])), [projects]);
  const projectTasks = useProjectTasks(projectId, items);
  const project = projectId ? (projectsById.get(projectId) ?? null) : null;
  const [editingId, setEditingId] = useState<string | null>(null);
  const editing = items.find((i) => i.id === editingId) ?? null;
  const shownError = error ?? projectsError;

  const isOpen = (i: Item) => i.status === "open";
  const counts = {
    today: today ? today.overdue.length + today.today.filter(isOpen).length : 0,
    all: items.filter(isOpen).length,
  };

  const handlers: ItemHandlers = {
    onOpen: (item) => {
      clearError();
      setEditingId(item.id);
    },
    onToggle: toggle,
    onRemove: remove,
  };

  return (
    <div className="flex h-screen">
      <Sidebar
        screen={screen}
        counts={counts}
        projects={projects}
        onNavigate={setScreen}
        onCreateProject={async (name) => {
          const created = await createProject(name);
          if (created) setScreen({ kind: "project", id: created.id });
          return created !== null;
        }}
      />
      <main className="flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-2xl flex-col gap-4 p-6">
          {shownError && !editing && <p className="text-sm text-destructive">{shownError}</p>}

          {screen.kind === "project" ? (
            project && (
              <ProjectView
                key={project.id}
                project={project}
                tasks={projectTasks}
                today={today?.date ?? null}
                onAdd={(title) => add(title, null, project.id)}
                onDoToday={(item) => scheduleToday(item.id)}
                {...handlers}
              />
            )
          ) : (
            <>
              <header>
                <h1 className="text-2xl font-semibold">
                  {VIEWS.find((v) => v.id === screen.view)?.label}
                </h1>
                {screen.view === "today" && today && (
                  <p className="text-sm text-muted-foreground">{formatLongDate(today.date)}</p>
                )}
              </header>

              <AddItemForm
                onAdd={(title) =>
                  add(title, screen.view === "today" ? (today?.date ?? null) : null)
                }
              />

              {screen.view === "all" && (
                <ItemList items={items} projects={projectsById} {...handlers} />
              )}

              {screen.view === "today" && today && (
                <>
                  {today.overdue.length > 0 && (
                    <Section title="Overdue">
                      <ItemList
                        items={today.overdue}
                        overdue
                        projects={projectsById}
                        {...handlers}
                      />
                    </Section>
                  )}
                  <Section title={today.overdue.length > 0 ? "Today" : undefined}>
                    <ItemList items={today.today} projects={projectsById} {...handlers} />
                  </Section>
                </>
              )}
            </>
          )}
        </div>
      </main>
      <TaskEditor
        item={editing}
        error={shownError}
        projects={projects}
        onSave={update}
        onTurnIntoProject={async (item) => {
          const result = await turnTaskIntoProject(item.id);
          if (!result) return;
          replace(result.task);
          setEditingId(null);
          setScreen({ kind: "project", id: result.project.id });
        }}
        onClose={() => setEditingId(null)}
      />
    </div>
  );
}

function formatLongDate(date: string) {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year, month - 1, day).toLocaleDateString(undefined, {
    weekday: "long",
    day: "numeric",
    month: "long",
  });
}
