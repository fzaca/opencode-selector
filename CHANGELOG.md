# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-06-13

### Features

- Redesign main UI layout and components
## [0.1.0] - 2026-06-13

### Bug Fixes

- Handle null project names
- Extract preview from part table
- Use passed theme in status bar
- Migrate legacy Inbox folder to All

### Documentation

- Add agent-facing project guidelines
- Add project overview and usage
- Add contribution guidelines and code of conduct
- Add initial changelog
- Correct CLI examples
- Update usage for global mode and optional folders
- Update unreleased section

### Features

- Add path resolution module
- Add clap-based command-line parser
- Add session repository and models
- Add sidecar folder store
- Add opencode process launcher
- Implement main session selector interface
- Implement permanent session delete with confirmation
- Add gg/G vim navigation
- Filter sessions by selected folder
- Filter sessions by current project directory
- Add folders_enabled setting
- Default to All/Archive and remove emojis
- Attach project directory to sessions
- Add --folders, --global flags and launch-in-dir
- Make folders optional and add global mode

### Miscellaneous Tasks

- Bootstrap Rust project with dependencies
- Add rustfmt and git-cliff configuration
- Enable chrono serde and clap env features

### Styling

- Apply cargo fmt
- Apply cargo fmt to remaining modules

### Ci

- Add GitHub Actions workflows
