export const sectionHeading = "text-xs font-medium tracking-wide text-muted-foreground uppercase";

export function Section({ title, children }: { title?: string; children: React.ReactNode }) {
  return (
    <section className="flex flex-col gap-2">
      {title && <h2 className={sectionHeading}>{title}</h2>}
      {children}
    </section>
  );
}
