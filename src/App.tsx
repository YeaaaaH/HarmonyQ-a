import { useState } from "react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList } from "@/components/ItemList";
import { TaskEditor } from "@/components/TaskEditor";
import { Sidebar } from "@/components/shell/Sidebar";
import { VIEWS, type View } from "@/components/shell/views";
import { useItems } from "@/hooks/useItems";
import { useSystemTheme } from "@/hooks/useSystemTheme";
import { useToday } from "@/hooks/useToday";
import type { Item } from "@/types/item";

export default function App() {
  useSystemTheme();
  const [view, setView] = useState<View>("today");
  const { items, error, clearError, add, update, toggle, remove } = useItems();
  const today = useToday(items);
  const [editingId, setEditingId] = useState<string | null>(null);
  const editing = items.find((i) => i.id === editingId) ?? null;

  const isOpen = (i: Item) => i.status === "open";
  const counts = {
    today: today ? today.overdue.length + today.today.filter(isOpen).length : 0,
    all: items.filter(isOpen).length,
  };
  const title = VIEWS.find((v) => v.id === view)?.label;

  const listProps = {
    onOpen: (item: Item) => {
      clearError();
      setEditingId(item.id);
    },
    onToggle: toggle,
    onRemove: remove,
  };

  return (
    <div className="flex h-screen">
      <Sidebar active={view} counts={counts} onSelect={setView} />
      <main className="flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-2xl flex-col gap-4 p-6">
          <header>
            <h1 className="text-2xl font-semibold">{title}</h1>
            {view === "today" && today && (
              <p className="text-sm text-muted-foreground">{formatLongDate(today.date)}</p>
            )}
          </header>

          <AddItemForm
            onAdd={(title) => add(title, view === "today" ? (today?.date ?? null) : null)}
          />
          {error && !editing && <p className="text-sm text-destructive">{error}</p>}

          {view === "all" && <ItemList items={items} {...listProps} />}

          {view === "today" && today && (
            <>
              {today.overdue.length > 0 && (
                <Section title="Overdue">
                  <ItemList items={today.overdue} overdue {...listProps} />
                </Section>
              )}
              <Section title={today.overdue.length > 0 ? "Today" : undefined}>
                <ItemList items={today.today} {...listProps} />
              </Section>
            </>
          )}
        </div>
      </main>
      <TaskEditor
        item={editing}
        error={error}
        onSave={update}
        onClose={() => setEditingId(null)}
      />
    </div>
  );
}

function Section({ title, children }: { title?: string; children: React.ReactNode }) {
  return (
    <section className="flex flex-col gap-2">
      {title && (
        <h2 className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
          {title}
        </h2>
      )}
      {children}
    </section>
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
