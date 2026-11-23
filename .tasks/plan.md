# Plan

Each step ends with using the app for a few days before starting the next.

## Step 0 - Walking skeleton (this scaffold)
- [x] Tauri + React + layered Rust backend
- [x] SQLite `items` table with migrations
- [x] List / add / toggle / delete on one screen
- [x] `cargo test` green
- [ ] First run on Windows (`npm run tauri dev`)

## Step 1 - Infrastructure
Reference: cURLyQ (`.github/workflows/`, `docs/releasing.md`).
- [x] Final `identifier`: `harmonyq-a` (sets the data dir; never change after first install)
- [x] `.gitattributes` (LF line endings)
- [x] CI: frontend build, `cargo fmt`/`clippy`/`test`, `cargo check` on Windows, macOS, Linux
- [x] Release on `v*.*.*` tag via tauri-action: Windows `.msi`, macOS `.dmg`, Linux `.AppImage`/`.deb` (draft release)
- [x] `docs/releasing.md`; switch to feature branches + PRs
- [x] First green CI run
- [x] Feature branches: task PRs skip the per-OS build check
- [ ] First draft release `v0.1.0`
- [ ] Real app icon (`npx tauri icon`)

## Step 2 - Daily use
UI: shadcn/ui + lucide + theme tokens, same conventions as cURLyQ. Layout: sidebar (Today, Routines, Tasks, Ideas, Promises, Settings) + main list with quick-add.
Each `###` below is one feature branch (`/feature start <slug>`); each checkbox is roughly one task PR.

### Daily tasks (`feature/daily-tasks`)
One-off tasks for a given day. Sidebar: Today, All tasks (other sections arrive with their features).
- [x] shadcn/ui setup, theme tokens
- [x] App shell: sidebar + main area, theme follows the OS
- [x] Migration + model: due date, time, note, priority; edit command
- [ ] Task editor: date, time, note, priority
- [ ] Today view: overdue (carried over, marked) + today, sorted by time and priority
- [ ] Reminders: OS notification at the task's time while the app runs

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
Global quick-capture hotkey, DB backup (`VACUUM INTO`), attachments (folder / link / chat), reminders when the app is closed (tray), weekly review, tags, generated TS types (tauri-specta), custom title bar, code signing.
