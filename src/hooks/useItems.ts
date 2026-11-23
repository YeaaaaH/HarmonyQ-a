import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/api/items";
import type { Item, ItemDetails } from "@/types/item";

export function useItems() {
  const [items, setItems] = useState<Item[]>([]);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async (action: () => Promise<void>): Promise<boolean> => {
    try {
      setError(null);
      await action();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }, []);

  useEffect(() => {
    run(async () => setItems(await api.listItems()));
  }, [run]);

  const replace = (updated: Item) =>
    setItems((prev) => prev.map((i) => (i.id === updated.id ? updated : i)));

  const add = (title: string) =>
    run(async () => {
      const created = await api.createItem(title);
      setItems((prev) => [created, ...prev]);
    });

  const update = (id: string, details: ItemDetails) =>
    run(async () => replace(await api.updateItem(id, details)));

  const toggle = (item: Item) =>
    run(async () => {
      const next = item.status === "open" ? "done" : "open";
      replace(await api.setItemStatus(item.id, next));
    });

  const remove = (id: string) =>
    run(async () => {
      await api.deleteItem(id);
      setItems((prev) => prev.filter((i) => i.id !== id));
    });

  return { items, error, clearError: () => setError(null), add, update, toggle, remove };
}
