# Contributing to opencode-selector

Thanks for your interest in contributing! This project is open source and
welcomes bug reports, feature suggestions, documentation improvements, and code.

## Quick start

1. Fork the repository.
2. Create a branch from `master`:
   - `feat/<short-name>` for features
   - `fix/<short-name>` for bug fixes
   - `docs/<short-name>` for documentation
   - `refactor/<short-name>` for refactors
3. Make your change.
4. Run the local checks:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
5. Commit using [Conventional Commits](https://www.conventionalcommits.org/).
6. Open a pull request.

## Commit rules

**One change = one commit.** Do not batch unrelated changes in a single commit.

Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `perf`.

Format: `<type>(<scope>): <imperative description>`.

Examples:

- `feat(db): add SessionRepository with list query`
- `fix(tui): handle empty session list gracefully`
- `docs(readme): add installation instructions`
- `chore(ci): add github actions workflow`

## Code style

- Rust 2024 edition, MSRV 1.85.
- English for all code, comments, docs, and commits.
- Use `?` propagation; avoid `unwrap()`/`expect()` in production code.
- Keep functions small and focused.
- Run `cargo fmt` before committing.

## Testing

- Add unit tests next to the code in `src/`.
- Use an in-memory SQLite database for DB tests.
- Use `tempfile` for filesystem-related tests.
- Ensure `cargo test` passes locally before pushing.

## Pull request process

- Keep PRs focused on a single concern.
- Reference related issues when applicable.
- Respond to review feedback promptly.
- A maintainer will merge once CI passes and the PR is approved.

## Release process

Releases are handled by maintainers:

1. Update version in `Cargo.toml`.
2. Run `git-cliff -o CHANGELOG.md`.
3. Commit `chore(release): bump version to vX.Y.Z`.
4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`.
5. Push `master` and tags.
6. GitHub Actions builds release artifacts.

## Questions?

Open an issue or start a discussion. We are happy to help.
