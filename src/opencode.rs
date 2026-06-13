use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// Launch opencode with an existing session ID.
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

/// Launch opencode with a session ID after changing to the given directory.
pub fn launch_session_in_dir(session_id: &str, dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();
    let status = Command::new("opencode")
        .current_dir(dir)
        .arg("-s")
        .arg(session_id)
        .status()
        .with_context(|| {
            format!(
                "failed to launch opencode for session {} in {}",
                session_id,
                dir.display()
            )
        })?;

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
