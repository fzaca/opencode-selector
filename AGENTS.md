# AGENTS.md — opencode-selector

> Agent-facing instructions for building and maintaining `opencode-selector`.
> This file is the source of truth for code, commits, and collaboration.

## 1. Project Vision

`opencode-selector` is an open-source TUI that replaces opencode's bare-bones
session picker with a fast, beautiful, keyboard-driven interface.

- Binary name: `opcs` (short), project name: `opencode-selector`.
- Built in Rust with `ratatui` + `crossterm`.
- Reads opencode's SQLite database, never corrupts it.
- Uses a sidecar file for folders/tags so opencode updates don't break us.
- Feels native: terminal-adaptive colors, arrow keys, vim keys, and mouse.

## 2. Tech Stack

| Layer | Crate / Tool |
|-------|--------------|
| TUI | `ratatui` 0.29 + `crossterm` 0.28 |
| CLI | `clap` derive |
| Errors | `anyhow` + `thiserror` |
| SQLite | `rusqlite` (bundled) |
| Config/Paths | `directories` + `toml` |
| Search | `nucleo` |
| Async (launcher only) | `tokio` |
| HTTP (future) | `reqwest` |
| Tests | `tempfile`, built-in test harness |

## 3. Architecture Rules

```
┌─────────────┐     ┌─────────────┐     ┌─────────────────────┐
│ cli (clap)  │────▶│ tui (app)   │────▶│ db / folders / opencode │
└─────────────┘     └─────────────┘     └─────────────────────┘
```

- **`src/db/`** is the only place that touches opencode's SQLite DB.
- **`src/folders/`** is the only place that touches the sidecar TOML.
- **`src/tui/`** owns all rendering and input state; it calls into `db` and `folders`.
- **`src/opencode.rs`** is the only place that executes the `opencode` binary.
- **`src/config.rs`** resolves paths and user settings.
- No `unwrap()` or `expect()` in production code except in `main()` or tests.

## 4. Code Style

- Rust 2024 edition, MSRV 1.85.
- Run `cargo fmt` and `cargo clippy -- -D warnings` before every commit.
- English for all code, comments, docs, and commits.
- Use `?` propagation; prefer `Result<T, E>` over panics.
- Keep functions small and focused.
- Use `tracing`-style log macros only after we add a logging crate.

## 5. Commit Rules — ONE CHANGE = ONE COMMIT

This is mandatory. Do not batch unrelated changes.

- Follow **Conventional Commits**.
- Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `perf`.
- Format: `<type>(<scope>): <imperative description>`.
- Examples:
  - `feat(db): add SessionRepository with list query`
  - `fix(tui): handle empty session list gracefully`
  - `docs(readme): add installation instructions`
  - `chore(ci): add github actions workflow`

If a PR would mix a feature and a fix, split it into multiple commits.

## 6. Branch Model

- Main branch: `master`.
- Feature branches: `feat/<short-name>`.
- Fix branches: `fix/<short-name>`.
- Docs branches: `docs/<short-name>`.
- Refactor branches: `refactor/<short-name>`.
- Keep branches short-lived. Rebase on `master` before opening a PR.

## 7. Changelog

We use `git-cliff` with `cliff.toml`.

- Do not hand-edit `CHANGELOG.md` between releases.
- During a release, run `git-cliff -o CHANGELOG.md` and commit the result.
- Releases follow SemVer: `v0.1.0`, `v0.2.0`, etc.

## 8. Testing

- Unit tests live next to the code in `src/`.
- DB tests use an in-memory SQLite database with fixtures.
- Folder store tests use `tempfile`.
- TUI tests exercise state transitions, not rendering.
- CI runs: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.

## 9. Security & Compatibility

- Never read `~/.local/share/opencode/auth.json`.
- Never write to opencode's DB except for safe, documented operations
  (e.g., renaming a session title via `UPDATE session SET title = ?`).
- Degrades gracefully if opencode's schema changes: read only known columns,
  treat missing fields as optional.
- Linux is the primary target for v0.1.

## 10. UI/UX Conventions

- Terminal-adaptive colors (16-color palette). No hardcoded solarized/catppuccin.
- Support three input modes: arrow keys, vim keys (`hjkl`), and mouse.
- `Enter` selects/open; `Esc`/`q` goes back or quits; `/` searches; `?` shows help.
- Preview is a full-screen modal, not a permanent side panel.
- Confirmation for destructive actions (delete).
- Status bar shows current mode and available shortcuts.

## 11. Folder System

- Sidecar file: `~/.config/opencode-selector/folders.toml`.
- A session can belong to one folder path, e.g., `"work/clients/acme"`.
- Default folders: `Inbox`, `Archive`.
- Folder operations are local metadata only; they do not move files on disk.

## 12. Release Checklist

1. Update version in `Cargo.toml`.
2. Run `git-cliff -o CHANGELOG.md`.
3. Commit `chore(release): bump version to vX.Y.Z`.
4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`.
5. Push `master` and tags.
6. GitHub Actions builds release artifacts.

## 13. Project Skills

Opencode-specific skills live in `skills/`:

- `opencode-selector-dev` — standard development workflow.
- `opencode-selector-release` — release workflow.
- `opencode-selector-ui-review` — UI/UX review checklist.
- `opencode-selector-compat` — opencode DB schema compatibility.

Load the relevant skill before starting work in that area.

## 14. Communication

- English only for code, comments, and commits.
- Keep PR descriptions actionable and reference issues.
- When in doubt, prefer simplicity over cleverness.
