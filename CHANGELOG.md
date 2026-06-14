# Changelog

## [0.5.0] - 2026-06-13

### Features
- Lazy-load session messages on preview open
- Full preview with all conversation messages and role headers
- Scrollable preview with keyboard (↑/↓/k/j, PgUp/PgDn, Home/End) and mouse
- Mouse scroll support in session list and preview
- Visual scrollbar on session list
- Selected session indicator (▶)
- Improved status bar with position counter, sort mode, search/project info
- Consistent background fill across all screens

### Bug Fixes
- Remove unused methods from Theme ([913e032](913e032))

## [0.4.0] - 2026-06-13

### Features
- Add command mode with fuzzy autocomplete (`:` prefix) ([e94c5ad](e94c5ad))

### Bug Fixes
- Execute command on Enter in command mode instead of only accepting suggestion ([e94c5ad](e94c5ad))
- Fill command bar background with theme color to avoid visual artifacts ([1d97a6e](1d97a6e))
- Prevent empty suggestion area from showing wrong background ([937714d](937714d))
- Fill entire screen background with theme color ([e94c5ad](e94c5ad))

## [0.3.0] - 2026-06-13

### Features

- Load custom colors from config.toml
- Load active opencode theme from tui.json
- Fetch built-in opencode themes from GitHub

### Refactor

- Hide mode badge in normal state
- Simplify status bar context labels
- Restore filter prefix in status bar
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
