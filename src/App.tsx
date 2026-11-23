import { AddItemForm } from "@/components/AddItemForm";
import { ItemList } from "@/components/ItemList";
import { useItems } from "@/hooks/useItems";

export default function App() {
  const { items, error, add, toggle, remove } = useItems();

  return (
    <main className="mx-auto flex max-w-xl flex-col gap-4 p-6">
      <h1 className="text-2xl font-semibold">HarmonyQ'a</h1>
      <AddItemForm onAdd={add} />
      {error && <p className="text-sm text-destructive">{error}</p>}
      <ItemList items={items} onToggle={toggle} onRemove={remove} />
    </main>
  );
}
