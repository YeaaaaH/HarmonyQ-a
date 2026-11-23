import { useCallback, useEffect, useState } from "react";
import * as api from "@/lib/api/items";
import type { Item, TodayView } from "@/types/item";

/** Re-reads the Today view whenever `items` change and when the window regains focus (the date may have rolled over). */
export function useToday(items: Item[]) {
  const [view, setView] = useState<TodayView | null>(null);

  const refresh = useCallback(() => {
    api.listToday().then(setView, () => setView(null));
  }, []);

  useEffect(refresh, [items, refresh]);

  useEffect(() => {
    window.addEventListener("focus", refresh);
    return () => window.removeEventListener("focus", refresh);
  }, [refresh]);

  return view;
}
