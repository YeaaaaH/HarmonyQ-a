# Projects - design note

Why projects look the way they do. Each rule below comes from how attention and motivation work, not from taste.

## The problem

Paper notes with open tasks wear you out because every unfinished thing keeps a small part of your attention busy.
A list alone does not fix that. A project helps only if it answers "what do I do next here?" without making you think again.

## Psychology -> rules

| Effect | What it says | Rule for HarmonyQ'a |
|---|---|---|
| **Zeigarnik effect / open loops** | Unfinished tasks keep popping into your head. | Getting a thought into a project must take seconds: quick-add inside the project, title only. |
| **Masicampo & Baumeister (2011)** | Those intrusive thoughts stop once there is a *concrete plan*, even before the task is done. | Every active project shows its **next step**. A project with no open task is flagged "needs a next step", the only warning a project gets. |
| **Implementation intentions (Gollwitzer)** | "When X, I will do Y" doubles follow-through. | One click puts the next step on Today, or on a date/time (reuses due date + reminders). |
| **Hick's law, cognitive load** | More choices, slower and more tiring decisions. | No limit on projects: a project may be 2 steps or 20, and there may be many. Instead the app makes the choice for you: projects are **sorted by their next step** (due soonest first, then recently touched), so the top of the list is what matters now. *Paused* projects are hidden from the sidebar. |
| **Ovsiankina effect, resumption cost** | We want to resume interrupted work, but getting back in costs effort. | A **"where I left off"** note per project, shown at the top of the project view. Write it when you stop, read it when you come back. |
| **Progress principle (Amabile)** | Visible small wins are the strongest day-to-day motivator. | Show "done this week" per project and a collapsible list of done tasks. Never delete done tasks silently. Finished projects go to a **Done shelf**: one card each with name, how long it ran and step count, so months of finished work stay visible. |
| **Goal gradient** | Effort grows as the goal feels close. | A **pulse** instead of a progress bar: a small sparkline of steps done per week over the last 4 weeks. It shows momentum, and adding steps never makes it go backwards. |
| **Ostrich effect, shame avoidance** | Red overdue piles make people stop opening the app. | No red for stale projects. A project you leave alone **slowly fades** (less saturation and opacity, like dust on paper), fully faded at 14 days; touching it restores it. A faded project's view offers "Pick a next step, or pause it?". |
| **Clear goal (SMART "done when")** | A vague project never feels finished. | An optional **"Done when..."** line under the project name. |
| **Closure** | Finishing explicitly frees the mind; abandoned things keep nagging. | "Complete project" is a deliberate action. If tasks are still open: move them to *No project*, or mark them done. Paused and done are both legitimate endings. Ticking a task draws an **ink strikethrough** (~300 ms) before it moves to Done: the one good thing about paper, kept. |
| **Pre-attentive recognition** | Color is noticed before text is read. | Each project gets an **automatic color**, never chosen by hand (no decision). Tasks show a small dot in their project's color on Today / All tasks. |
| **Planning fallacy** | Self-set deadlines are almost always wrong. | Project deadline is optional; tasks keep their own dates. |

## Model

- New table `projects`: `id`, `name`, `outcome` ("done when", may be empty), `status` (`active` / `paused` / `done`), `resume_note`, `hue` (0-359, from the name at creation and stored, so renaming keeps the color), `created_at`, `updated_at`, `completed_at`.
- `items.project_id` nullable, `ON DELETE SET NULL`. Tasks without a project stay exactly as today.
- `items.completed_at` (ms, set when a task is ticked, cleared when unticked): needed for the pulse, "done this week" and the Done shelf. Tasks ticked before this migration have none and just do not count.
- **Freshness is derived**: from the project's last activity (task added, ticked or edited, note changed), 1.0 at 0 days down to 0.0 at 14 days. The UI maps it to saturation and opacity.
- **Next step is derived, not stored**: the first open task of the project, by due date, then time, then priority, then oldest. No extra state to keep in sync.
- Rules live in `ProjectService`: ordering by next step, "needs next step", freshness, weekly pulse counts, complete with open tasks.

## UI

- Sidebar: a **Projects** section under Today / All tasks with the active projects in next-step order (open task count as a badge) and a "+" to create one. The section scrolls on its own, so a long list never pushes Today away. Paused and done projects sit behind "Show all".
- **Project view**, top to bottom:
  1. Name with its color, "Done when...", pulse sparkline, status menu (pause / complete).
  2. **Next step** card, the most prominent element, with "Do today" and a date picker.
  3. "Where I left off" note (inline editable, one-line preview).
  4. Open tasks + quick-add (new tasks go into this project).
  5. Done tasks, collapsed, with "N done this week".
- Task editor: a project picker, and **"Turn into project"**: creates a project named after the task, the task stays inside it as the first step (you often see a task has steps only once you start it).
- Today / All tasks: a color dot + muted project name on each task, so the context stays visible without a separate screen.
- **Done shelf**: reached from "Show all" in the Projects section; finished projects as calm cards, newest first.
- Project color: hue from the stored `hue`, lightness and chroma fixed per theme (oklch) so every project color fits the palette in light and dark.
- Calm visuals: muted tokens, no red outside real overdue tasks, no badges that count *undone* things on paused projects.

## Out of scope (until daily use asks)

Sub-projects, choosing colors or icons by hand, a kanban board, time tracking, `#project` syntax in quick-add, weekly review screen.
