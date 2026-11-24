import { useState, type FormEvent } from "react";
import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

interface Props {
  onAdd: (title: string) => Promise<boolean>;
  placeholder?: string;
}

export function AddItemForm({ onAdd, placeholder = "What's on your mind?" }: Props) {
  const [title, setTitle] = useState("");

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    if (!title.trim()) return;
    if (await onAdd(title)) setTitle("");
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-2">
      <Input
        autoFocus
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder={placeholder}
      />
      <Button type="submit">
        <Plus />
        Add
      </Button>
    </form>
  );
}
