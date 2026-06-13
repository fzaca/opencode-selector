---
name: opencode-selector-release
description: Release workflow for opencode-selector. Bump version, generate changelog, tag, and push.
---

# opencode-selector-release

Use this skill when preparing a new release of `opencode-selector`.

## Trigger

- User asks to release a new version.
- User mentions `git-cliff`, `CHANGELOG.md`, or version bump.

## Workflow

1. **Determine version**
   - Use SemVer: `v0.X.Y`.
   - Default to bumping the minor version for new features, patch for fixes.

2. **Update `Cargo.toml`**
   - Set `version` to the new version without the `v` prefix.

3. **Generate changelog**
   - Run `git-cliff -o CHANGELOG.md`.
   - Review the generated file.

4. **Commit and tag**
   - Commit with message `chore(release): bump version to vX.Y.Z`.
   - Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`.

5. **Push**
   - Push `master`.
   - Push tags: `git push origin --tags`.

6. **Verify CI**
   - Ensure the release workflow runs and publishes artifacts.

## Constraints

- Do not hand-edit `CHANGELOG.md` except through `git-cliff`.
- Ensure `cargo test` and `cargo clippy -- -D warnings` pass before tagging.
