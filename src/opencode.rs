use std::process::Command;

use anyhow::{Context, Result};

/// Launch opencode with an existing session ID.
///
/// This uses `exec` semantics: it replaces the current process with opencode
/// so the TUI is cleanly torn down before the editor starts.
pub fn launch_session(session_id: &str) -> Result<()> {
    let status = Command::new("opencode")
        .arg("-s")
        .arg(session_id)
        .status()
        .with_context(|| format!("failed to launch opencode for session {}", session_id))?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("opencode exited with status {:?}", status.code())
    }
}

/// Launch opencode to create a new session.
pub fn launch_new() -> Result<()> {
    let status = Command::new("opencode")
        .status()
        .context("failed to launch opencode")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("opencode exited with status {:?}", status.code())
    }
}
