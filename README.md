# opencode-selector

> A beautiful, keyboard-driven TUI for selecting, organizing, and launching [opencode](https://opencode.ai) sessions.

`opencode-selector` (binary: `opcs`) reads your local opencode database and gives you a fast, intuitive interface to browse, search, preview, and open your sessions. It also adds a folder system to organize conversations without touching opencode's internals.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)

## Features

- 🚀 **Fast session switcher** — fuzzy search, sort by date/name, instant launch.
- 📁 **Folder system** — organize sessions into folders via a safe sidecar file.
- 🖥️ **Native terminal feel** — adaptive 16-color theme, mouse, arrow keys, and vim bindings.
- 🔍 **Full-screen preview** — peek at a session before opening it.
- 🛡️ **Read-only by default** — we only touch opencode's DB for explicit user actions like rename.
- ⌨️ **Multiple control schemes** — arrows, `hjkl`, mouse, and standard shortcuts.

## Installation

### Quick install (Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/fzaca/opencode-selector/master/scripts/install.sh | bash
```

This downloads the latest prebuilt binary for your architecture (x86_64 or ARM64) and installs it to `~/.local/bin/opcs` or `/usr/local/bin/opcs`.

### Upgrade

The installed binary includes an upgrade command:

```bash
opcs upgrade
```

Or re-run the install script — it always fetches the latest version.

### From source

```bash
cargo install --path .
```

The binary is installed as `opcs`.

### Prebuilt binaries

Download the latest release from the [releases page](https://github.com/fzaca/opencode-selector/releases).

## Usage

```bash
# Open the session selector for the current project
opcs

# Show all sessions across all projects
opcs --global

# Enable folders
opcs --folders

# Launch a specific session directly
opcs session ses_xxx

# List sessions as JSON
opcs list

# Show help
opcs --help
```

## Default keybindings

| Key | Action |
|-----|--------|
| `↑` / `↓` or `k` / `j` | Move selection |
| `Enter` | Open selected session in opencode |
| `/` | Search/filter sessions |
| `p` | Open full-screen preview |
| `n` | Create a new opencode session |
| `r` | Rename session |
| `m` | Move session to folder (folders enabled) |
| `d` | Archive session |
| `D` | Delete session (with confirmation) |
| `P` | Toggle current project / all projects |
| `F` | Toggle folder system |
| `a` | Jump to All folder (folders enabled) |
| `N` | Create new folder (folders enabled) |
| `?` | Show help |
| `q` / `Esc` | Quit / go back |

Mouse interaction is also supported.

## Project-aware mode

When you run `opcs` inside a project directory, it shows only the sessions for
that project. Press `P` to switch to all projects.

## Global mode

Run `opcs --global` or `opcs global` to see every session across all projects.
In this mode folders are disabled, and opening a session changes to that
project's directory first.

## Folder system

The folder system is **disabled by default**. Enable it with `opcs --folders`
or by setting `folders_enabled = true` in
`~/.config/opencode-selector/config.toml`.

Folder metadata is stored in `~/.config/opencode-selector/folders.toml`,
completely separate from opencode's database.

Default folders (when enabled):

- `All`
- `Archive`

Create, rename, and delete custom folders directly from the TUI.

## Development

See [AGENTS.md](AGENTS.md) for the full contributor guide and project conventions.

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## License

MIT © Zacarias
