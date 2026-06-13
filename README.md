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

### From source

```bash
cargo install --path .
```

The binary is installed as `opcs`.

### Prebuilt binaries

Download the latest release from the [releases page](https://github.com/fzaca/opencode-selector/releases).

## Usage

```bash
# Open the session selector
opcs

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
| `m` | Move session to folder |
| `d` | Archive session |
| `D` | Delete session (with confirmation) |
| `P` | Toggle current project / all projects |
| `a` | Jump to All folder |
| `?` | Show help |
| `q` / `Esc` | Quit / go back |

Mouse interaction is also supported.

## Folder system

Sessions can be organized into folders. Folder metadata is stored in `~/.config/opencode-selector/folders.toml`, completely separate from opencode's database.

Default folders:

- `Inbox`
- `Archive`

Create, rename, and delete folders directly from the TUI.

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
