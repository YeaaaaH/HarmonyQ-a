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

### Layout (`feature/layout`)
- [ ] shadcn/ui setup, theme tokens
- [ ] Sidebar + main area

### Item types (`feature/item-types`)
- [ ] Migration: type, who, due date
- [ ] Type picker in quick-add, per-type lists

### Routines (`feature/routines`)
- [ ] Migration: `routines` + `routine_checks (routine_id, date)`
- [ ] Service: schedule, check/uncheck for a day
- [ ] Routines screen

### Today (`feature/today`)
- [ ] Today view: routines for today, tasks due today, promises due soon

### Search (`feature/search`)
- [ ] SQLite FTS5 index + search box

## Later (only if daily use demands it)
Global quick-capture hotkey, DB backup (`VACUUM INTO`), attachments (folder / link / chat), reminders, weekly review, tags, generated TS types (tauri-specta), custom title bar, code signing.
