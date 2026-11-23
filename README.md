# HarmonyQ'a

A calm, local desktop app for running your day: tasks, ideas, promises and routines in one place. Built with Tauri 2, React and Rust + SQLite. Everything stays on your machine.

## Development

Prerequisites: Node.js, Rust (stable) and the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```
npm install
npm run tauri dev      # app window with hot reload
npm run tauri build    # release binary / installer
cargo test             # backend tests, run from src-tauri/
```

See `docs/architecture.md` for how the code is organised and `.tasks/plan.md` for the roadmap.
