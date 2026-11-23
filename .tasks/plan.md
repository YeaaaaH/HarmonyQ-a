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
- [ ] First green CI run and first draft release `v0.1.0`
- [ ] Real app icon (`npx tauri icon`)

## Step 2 - Daily use
UI: shadcn/ui + lucide + theme tokens, same conventions as cURLyQ. Layout: sidebar (Today, Routines, Tasks, Ideas, Promises, Settings) + main list with quick-add.
- [ ] shadcn/ui setup, sidebar layout
- [ ] Item types: task, idea, promise (who + due date) - new migration
- [ ] Routines: `routines` + `routine_checks (routine_id, date)`, checks reset daily
- [ ] Today view: routines for today, tasks due today, promises due soon
- [ ] Search (SQLite FTS5)

## Later (only if daily use demands it)
Global quick-capture hotkey, DB backup (`VACUUM INTO`), attachments (folder / link / chat), reminders, weekly review, tags, generated TS types (tauri-specta), custom title bar, code signing.
