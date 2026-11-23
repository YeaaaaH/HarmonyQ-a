import { useState, type FormEvent } from "react";

interface Props {
  onAdd: (title: string) => Promise<void>;
}

export function AddItemForm({ onAdd }: Props) {
  const [title, setTitle] = useState("");

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    if (!title.trim()) return;
    await onAdd(title);
    setTitle("");
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-2">
      <input
        autoFocus
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="What's on your mind?"
        className="flex-1 rounded-md border border-stone-300 bg-white px-3 py-2 outline-none focus:border-stone-500"
      />
      <button
        type="submit"
        className="rounded-md bg-stone-800 px-4 py-2 text-white hover:bg-stone-700"
      >
        Add
      </button>
    </form>
  );
}
