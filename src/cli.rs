use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A beautiful TUI for selecting, organizing, and launching opencode sessions.
#[derive(Debug, Parser)]
#[command(name = "opcs")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to opencode's SQLite database.
    #[arg(long, global = true, env = "OPENCODE_DB_PATH")]
    pub db: Option<PathBuf>,

    /// Path to the folders sidecar file.
    #[arg(long, global = true, env = "OPENCODE_FOLDERS_PATH")]
    pub folders: Option<PathBuf>,

    /// Enable the folder system in the TUI.
    #[arg(long, global = true)]
    pub folders_enabled: bool,

    /// Show sessions from all projects and disable project filtering.
    #[arg(long, global = true)]
    pub global: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Launch a specific session by ID.
    Session {
        /// Session ID.
        id: String,
    },
    /// List sessions as JSON.
    List {
        /// Include archived sessions.
        #[arg(long)]
        archived: bool,
    },
    /// Open the selector in global mode (all projects, no folders).
    Global,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
