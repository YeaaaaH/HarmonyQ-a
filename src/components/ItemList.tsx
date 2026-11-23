import type { Item } from "../types/item";

interface Props {
  items: Item[];
  onToggle: (item: Item) => void;
  onRemove: (id: string) => void;
}

export function ItemList({ items, onToggle, onRemove }: Props) {
  if (items.length === 0) {
    return <p className="py-8 text-center text-stone-500">Nothing here yet.</p>;
  }

  return (
    <ul className="divide-y divide-stone-200 rounded-md border border-stone-200 bg-white">
      {items.map((item) => (
        <li key={item.id} className="flex items-center gap-3 px-3 py-2">
          <input
            type="checkbox"
            checked={item.status === "done"}
            onChange={() => onToggle(item)}
          />
          <span
            className={
              item.status === "done" ? "flex-1 text-stone-400 line-through" : "flex-1"
            }
          >
            {item.title}
          </span>
          <button
            onClick={() => onRemove(item.id)}
            className="text-sm text-stone-400 hover:text-red-600"
          >
            Delete
          </button>
        </li>
      ))}
    </ul>
  );
}
