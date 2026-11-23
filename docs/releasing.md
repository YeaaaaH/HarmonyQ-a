# Releasing

Two GitHub Actions workflows:

- [`ci.yml`](../.github/workflows/ci.yml) runs on every PR and push to `main`: frontend build, `cargo fmt --check`, `clippy -D warnings`, `cargo test`, and a `cargo check` on Windows, Linux and macOS (Intel + Apple Silicon).
- [`release.yml`](../.github/workflows/release.yml) runs on a pushed `v*.*.*` tag and builds installers with [`tauri-action`](https://github.com/tauri-apps/tauri-action) into a **draft** GitHub Release.

| OS | Installer |
|---|---|
| Windows | `.msi` |
| macOS (arm64, x64) | `.dmg` |
| Linux | `.AppImage`, `.deb` |

Formats are set by `bundle.targets` in `src-tauri/tauri.conf.json`; each OS builds only the ones that apply to it.

## Cutting a release

1. Bump the version in three places, kept in sync by hand:
   - `src-tauri/tauri.conf.json` (`version`)
   - `package.json` via `npm version X.Y.Z --no-git-tag-version` (also updates `package-lock.json`)
   - `src-tauri/Cargo.toml` (`[package].version`), then `cargo check` to sync `Cargo.lock`
2. Commit the bump on `main`.
3. Tag and push: `git tag vX.Y.Z && git push && git push --tags`.
4. Wait for the four `Release` jobs in the Actions tab.
5. Open the draft under Releases, check the installers, write notes, publish.

Local Windows build: `npm run tauri build` → `src-tauri/target/release/bundle/msi/`.

## Known limitations

- **Unsigned builds**: Windows SmartScreen and macOS Gatekeeper warn on first launch.
- **MSI installs per machine** and asks for admin rights.
- **OS floor is set by Tauri**: Windows 10 1803+, macOS 10.13+ (Intel) / 11+ (Apple Silicon), Ubuntu 22.04 / Debian 12+.
