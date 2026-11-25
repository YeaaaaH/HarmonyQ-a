import { projectColor } from "@/lib/projectColor";
import { cn } from "@/lib/utils";

export function ProjectDot({ hue, className }: { hue: number; className?: string }) {
  return (
    <span
      className={cn("size-2.5 shrink-0 rounded-full", className)}
      style={{ backgroundColor: projectColor(hue) }}
    />
  );
}
