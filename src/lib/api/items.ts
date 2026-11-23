import { invoke } from "@tauri-apps/api/core";
import type { Item, ItemStatus } from "../../types/item";

export const listItems = () => invoke<Item[]>("list_items");

export const createItem = (title: string) =>
  invoke<Item>("create_item", { request: { title } });

export const setItemStatus = (id: string, status: ItemStatus) =>
  invoke<Item>("set_item_status", { id, status });

export const deleteItem = (id: string) => invoke<void>("delete_item", { id });
