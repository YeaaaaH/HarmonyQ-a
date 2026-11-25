import { useCallback, useEffect, useState } from "react";
import { useAction } from "@/hooks/useAction";
import * as api from "@/lib/api/items";
import type { Item, ItemDetails } from "@/types/item";

export function useItems() {
  const [items, setItems] = useState<Item[]>([]);
  const { error, clearError, run } = useAction();

  const attempt = useCallback(
    async (action: () => Promise<void>) => (await run(action)) !== null,
    [run],
  );

  useEffect(() => {
    attempt(async () => setItems(await api.listItems()));
  }, [attempt]);

  const replace = useCallback(
    (updated: Item) => setItems((prev) => prev.map((i) => (i.id === updated.id ? updated : i))),
    [],
  );

  const add = (title: string, dueDate: string | null = null, projectId: string | null = null) =>
    attempt(async () => {
      const created = await api.createItem(title, dueDate, projectId);
      setItems((prev) => [created, ...prev]);
    });

  const update = (id: string, details: ItemDetails) =>
    attempt(async () => replace(await api.updateItem(id, details)));

  const scheduleToday = (id: string) =>
    attempt(async () => replace(await api.scheduleItemToday(id)));

  const toggle = (item: Item) =>
    attempt(async () => {
      const next = item.status === "open" ? "done" : "open";
      replace(await api.setItemStatus(item.id, next));
    });

  const remove = (id: string) =>
    attempt(async () => {
      await api.deleteItem(id);
      setItems((prev) => prev.filter((i) => i.id !== id));
    });

  return { items, error, clearError, replace, add, update, scheduleToday, toggle, remove };
}
