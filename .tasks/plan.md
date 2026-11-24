# Plan

Each step ends with using the app for a few days before starting the next.

A task that touches both frontend and backend is split into vertical slices, not layers: each PR carries the backend it needs plus the UI for it, so it can be tried in the running app right away. No backend-only PR without a screen to test it from.

## Step 0 - Walking skeleton (this scaffold)
- [x] Tauri + React + layered Rust backend
- [x] SQLite `items` table with migrations
- [x] List / add / toggle / delete on one screen
- [x] `cargo test` green
- [x] First run on Windows (`npm run tauri dev`)

## Step 1 - Infrastructure
Reference: cURLyQ (`.github/workflows/`, `docs/releasing.md`).
- [x] Final `identifier`: `harmonyq-a` (sets the data dir; never change after first install)
- [x] `.gitattributes` (LF line endings)
- [x] CI: frontend build, `cargo fmt`/`clippy`/`test`, `cargo check` on Windows, macOS, Linux
- [x] Release on `v*.*.*` tag via tauri-action: Windows `.msi`, macOS `.dmg`, Linux `.AppImage`/`.deb` (draft release)
- [x] `docs/releasing.md`; switch to feature branches + PRs
- [x] First green CI run
- [x] Feature branches: task PRs skip the per-OS build check
- [x] First draft release `v0.1.0`
- [x] Real app icon (`npx tauri icon`, source `src-tauri/icons/app-icon.svg`)

## Step 2 - Daily use
UI: shadcn/ui + lucide + theme tokens, same conventions as cURLyQ. Layout: sidebar (Today, Routines, Tasks, Ideas, Promises, Settings) + main list with quick-add.
Each `###` below is one feature branch (`/feature start <slug>`); each checkbox is roughly one task PR.

### Daily tasks (`feature/daily-tasks`)
One-off tasks for a given day. Sidebar: Today, All tasks (other sections arrive with their features).
- [x] shadcn/ui setup, theme tokens
- [x] App shell: sidebar + main area, theme follows the OS
- [x] Migration + model: due date, time, note, priority; edit command
- [x] Task editor: date, time, note, priority
- [x] Today view: overdue (carried over, marked) + today, sorted by time and priority
- [x] Reminders: OS notification at the task's time while the app runs (and hidden in the tray)
- [x] Tray: closing the window hides the app to the tray; single instance

### Projects (`feature/projects`)
Group tasks into projects, each with a visible next step. Design and reasoning: `.tasks/projects.md`.
- [ ] BE only (written before the vertical-slice rule): migration + model + service + commands for `projects` (with `hue`), `items.project_id`, `items.completed_at`
- [ ] BE+FE: TS types and API wrappers, sidebar Projects section, project view with quick-add, next step card, project colors
- [ ] BE+FE: project picker in task editor, color dot in Today / All tasks, turn a task into a project
- [ ] BE+FE: lifecycle (pause / complete with open tasks handled), fading stale projects, Done shelf
- [ ] BE+FE: "Where I left off" note, done-this-week, pulse sparkline, ink strikethrough

### Brain dump (`feature/brain-dump`)
Empty your head first, organise later. Capture and sorting are separate modes.
- [ ] Full-screen capture: one thought per line, Enter for the next, no fields
- [ ] Sorting pass: one card at a time, one key each to a project, Today, someday or bin

### Shutdown (`feature/shutdown`)
Close the day's open loops before sleep.
- [ ] Evening tray prompt (time in Settings): "Anything still on your mind?" -> brain dump
- [ ] For projects touched today, offer a one-line "where I left off" note

### Item types (`feature/item-types`)
- [ ] Migration: type, who (due date comes from daily tasks)
- [ ] Type picker in quick-add, per-type lists

### Routines (`feature/routines`)
- [ ] Migration: `routines` + `routine_checks (routine_id, date)`
- [ ] Service: schedule, check/uncheck for a day
- [ ] Routines screen

### Today+ (`feature/today-plus`)
- [ ] Today view also shows routines and promises due soon

### Search (`feature/search`)
- [ ] SQLite FTS5 index + search box

## Later (only if daily use demands it)
Global quick-capture hotkey, DB backup (`VACUUM INTO`), attachments (folder / link / chat), weekly review, tags, generated TS types (tauri-specta), custom title bar, code signing.
