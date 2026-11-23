import { useCallback, useEffect, useState } from "react";
import * as api from "../lib/api/items";
import type { Item } from "../types/item";

export function useItems() {
  const [items, setItems] = useState<Item[]>([]);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async (action: () => Promise<void>) => {
    try {
      setError(null);
      await action();
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    run(async () => setItems(await api.listItems()));
  }, [run]);

  const add = (title: string) =>
    run(async () => {
      const created = await api.createItem(title);
      setItems((prev) => [created, ...prev]);
    });

  const toggle = (item: Item) =>
    run(async () => {
      const next = item.status === "open" ? "done" : "open";
      const updated = await api.setItemStatus(item.id, next);
      setItems((prev) => prev.map((i) => (i.id === updated.id ? updated : i)));
    });

  const remove = (id: string) =>
    run(async () => {
      await api.deleteItem(id);
      setItems((prev) => prev.filter((i) => i.id !== id));
    });

  return { items, error, add, toggle, remove };
}
