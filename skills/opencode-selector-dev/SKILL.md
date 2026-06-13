---
name: opencode-selector-dev
description: Standard development workflow for opencode-selector. Plan, implement, test, lint, and commit one change at a time.
---

# opencode-selector-dev

Use this skill when implementing features, fixes, refactors, or tests in the
`opencode-selector` Rust project.

## Trigger

- User asks to add, change, fix, or test code in `opencode-selector`.
- The task touches `src/`, `Cargo.toml`, tests, or project tooling.

## Workflow

1. **Understand**
   - Read `AGENTS.md`.
   - Check relevant existing modules and tests.
   - Identify the minimal change needed.

2. **Plan**
   - State the approach in one sentence.
   - Mention files that will change.

3. **Implement**
   - Write or modify Rust code following the project style.
   - Keep functions small and focused.
   - Use `Result` and `?`; avoid `unwrap()`/`expect()` outside `main()` and tests.

4. **Test**
   - Add or update unit tests next to the code.
   - Run `cargo test`.
   - Fix failures before proceeding.

5. **Lint & Format**
   - Run `cargo fmt`.
   - Run `cargo clippy -- -D warnings`.
   - Fix all warnings.

6. **Commit**
   - Follow Conventional Commits: `<type>(<scope>): <imperative description>`.
   - One logical change = one commit.
   - Do not batch unrelated changes.

## Constraints

- Do not read `~/.local/share/opencode/auth.json`.
- Only touch `~/.local/share/opencode/opencode.db` through `src/db/` repository code.
- Keep folder metadata in the sidecar file, never in opencode's DB.
- English only for code, comments, docs, and commits.
