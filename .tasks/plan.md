# Plan

Each step ends with using the app for a few days before starting the next.

## Step 0 - Walking skeleton (this scaffold)
- [x] Tauri + React + layered Rust backend
- [x] SQLite `items` table with migrations
- [x] List / add / toggle / delete on one screen
- [ ] First run on Windows (`npm install`, `npm run tauri dev`), `cargo test` green

## Step 1 - Capture
- [ ] Global hotkey opens a one-line quick-capture window
- [ ] Backup: copy the DB file (menu action)

## Step 2 - Make it useful
- [ ] Item types: task, idea, promise (who + due date) - new migration
- [ ] Today view
- [ ] Search (SQLite FTS5)

## Later (only if daily use demands it)
Attachments (folder / link / chat), recurring items, reminders, weekly review, tags, generated TS types (tauri-specta), CI (adapt cURLyQ's workflows), real icons, custom title bar, shadcn/ui.
