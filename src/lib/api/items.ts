import { invoke } from "@tauri-apps/api/core";
import type { Item, ItemDetails, ItemStatus, TodayView } from "@/types/item";

export const listItems = () => invoke<Item[]>("list_items");

export const listToday = () => invoke<TodayView>("list_today");

export const createItem = (
  title: string,
  dueDate: string | null = null,
  projectId: string | null = null,
) => invoke<Item>("create_item", { request: { title, dueDate, projectId } });

export const updateItem = (id: string, details: ItemDetails) =>
  invoke<Item>("update_item", { id, request: details });

export const setItemStatus = (id: string, status: ItemStatus) =>
  invoke<Item>("set_item_status", { id, status });

export const deleteItem = (id: string) => invoke<void>("delete_item", { id });
