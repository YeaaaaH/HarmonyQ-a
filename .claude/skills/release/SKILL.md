---
name: release
description: Cut a HarmonyQ'a release - bump the version in tauri.conf.json/package.json/Cargo.toml, commit to main, tag vX.Y.Z and push to trigger the installer builds (.github/workflows/release.yml). Use when the user says "release", "cut a release", "bump the version" or invokes /release.
---

# Release

Mechanizes `docs/releasing.md`; read it first if it's not in context and keep the two in sync.

This is the one flow that commits and pushes straight to `main`: a version bump isn't code under review, and the tag must point at a commit on `main`. Don't route it through a PR without asking the user.

## Steps

0. **On `main` and up to date?** `git branch --show-current`, `git status`, `git pull`. Not on `main`, or uncommitted changes besides the bump: stop and say so.
1. **Target version.** Read `version` from `src-tauri/tauri.conf.json`; check `package.json` and `src-tauri/Cargo.toml` match (drifted: stop and report).
   - Explicit version (`/release 0.3.0`): use it.
   - `patch` / `minor` / `major`: compute it.
   - Otherwise ask with AskUserQuestion, showing each option explicitly (e.g. `0.1.0 → 0.1.1`).
2. **Update.** Edit `version` in `src-tauri/tauri.conf.json` and `[package].version` in `src-tauri/Cargo.toml`. For `package.json` run `npm version X.Y.Z --no-git-tag-version` (also updates `package-lock.json`).
3. **Sync and check.** `cargo check` from `src-tauri/` (updates `Cargo.lock`), `npx tsc --noEmit`. Any failure: stop and report.
4. **Confirm with the user** (old → new version) before touching the remote: the push starts builds on three OSes and creates a draft GitHub Release.
5. **Commit, tag, push** after confirmation:
   - `git add src-tauri/tauri.conf.json package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock`
   - Commit subject `HQ-NNN Bump version to X.Y.Z`, NNN as in `.claude/agents/committer.md` (highest `HQ-` number in `git log --all` + 1). One line, no AI attribution.
   - `git tag vX.Y.Z`, then `git push && git push origin vX.Y.Z`
6. **Report** the Actions URL (`https://github.com/YeaaaaH/HarmonyQ-a/actions`) and remind the user the release is a **draft**: check the installers and publish it by hand.

## Guardrails

- Never force-push, amend or skip hooks.
- Never sweep unrelated changes into the release commit.
