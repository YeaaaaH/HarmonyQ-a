# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project

HarmonyQ'a is a local desktop app for managing daily things: tasks, ideas, promises, routines. It starts tiny and grows one step at a time; `.tasks/plan.md` is the roadmap. Name notes: display name `HarmonyQ'a`, repo/package `HarmonyQ-a` / `harmonyq-a` (an apostrophe is not valid in repo, package or path names).

## Stack

- Frontend: TypeScript + React 19 + Vite + Tailwind CSS v4, in Tauri's webview.
- Backend: Rust with Tauri commands, SQLite via `rusqlite` (bundled).
- Unlike cURLyQ, **business logic lives in Rust**; the frontend is a thin UI over typed `invoke` wrappers.

Read `docs/architecture.md` before changing layers, the database schema or the Rust-TS contract. If a change makes it inaccurate, update it in the same change.

- Dev: `npm run tauri dev`
- Frontend type check + build: `npm run build`
- Rust: `cargo check` / `cargo test` / `cargo clippy` (from `src-tauri/`)

## Workflow

Work step by step, one small piece at a time, with a review checkpoint before the next piece. Do not implement several features in one pass unless asked. A feature is only added once the previous step has been used in daily life.

When a Rust or TS concept appears for the first time, explain it briefly in the chat reply (not in code comments) - this is also a learning project.

## Code comments

Code explains itself through naming and structure. Comment only a non-obvious why (a workaround, a library quirk) in 1-2 lines. No history, no restating the code, no architecture prose.

## Git

`main` is the default branch. Nothing is pushed to `main` or `feature/*` directly; everything lands via PR once CI (`.github/workflows/ci.yml`) is green.

```
main <- feature/<slug> <- task/<slug>, bug/<slug>     (a larger change, several PRs)
main <- task/<slug>, bug/<slug>                        (a standalone small change)
```

- A feature groups several tasks into one logical change. Start and finish it with the `feature` skill (`/feature start <slug>`, `/feature finish`).
- Task and bug work uses the `commit` skill (`/commit`): it delegates to the `committer` subagent (`.claude/agents/committer.md`), which branches off the current feature (or `main`), commits, pushes and opens the PR into it.
- Merge task/bug PRs with **squash**, feature PRs into `main` with a **merge commit**.
- CI: PRs into a feature run the frontend and Rust checks only; PRs into `main` and pushes to `main` also run the per-OS build check.
- Releases use the `release` skill (`/release`), the only flow that commits to `main` directly; see `docs/releasing.md`.

Commit message: one line, `HQ-NNN` + at most 7 words briefly saying what changed (e.g. `HQ-002 Add routines table and service`). NNN is the next number after the last commit's, zero-padded to 3 digits. No body, no trailers.
