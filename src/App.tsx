import { useState } from "react";
import { AddItemForm } from "@/components/AddItemForm";
import { ItemList } from "@/components/ItemList";
import { TaskEditor } from "@/components/TaskEditor";
import { Sidebar } from "@/components/shell/Sidebar";
import { VIEWS, type View } from "@/components/shell/views";
import { useItems } from "@/hooks/useItems";
import { useSystemTheme } from "@/hooks/useSystemTheme";

export default function App() {
  useSystemTheme();
  const [view, setView] = useState<View>("all");
  const { items, error, clearError, add, update, toggle, remove } = useItems();
  const [editingId, setEditingId] = useState<string | null>(null);
  const editing = items.find((i) => i.id === editingId) ?? null;
  const openCount = items.filter((i) => i.status === "open").length;
  const title = VIEWS.find((v) => v.id === view)?.label;

  return (
    <div className="flex h-screen">
      <Sidebar active={view} counts={{ all: openCount }} onSelect={setView} />
      <main className="flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-2xl flex-col gap-4 p-6">
          <h1 className="text-2xl font-semibold">{title}</h1>
          {view === "all" ? (
            <>
              <AddItemForm onAdd={add} />
              {error && !editing && <p className="text-sm text-destructive">{error}</p>}
              <ItemList
                items={items}
                onOpen={(item) => {
                  clearError();
                  setEditingId(item.id);
                }}
                onToggle={toggle}
                onRemove={remove}
              />
            </>
          ) : (
            <p className="py-8 text-center text-muted-foreground">
              Tasks planned for today will show up here once tasks have dates.
            </p>
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
