use std::process::Command;

use anyhow::{Context, Result};

const REPO: &str = "fzaca/opencode-selector";

pub fn run() -> Result<()> {
    let current_exe = std::env::current_exe().context("failed to get current executable path")?;

    if current_exe.to_string_lossy().contains("target/debug") {
        anyhow::bail!(
            "You are running a development build. Use 'cargo install --path .' or the install script."
        );
    }

    println!("Checking for latest version...");

    let latest = latest_release_tag()?;
    let current = env!("CARGO_PKG_VERSION");

    if latest == format!("v{current}") {
        println!("Already up to date (v{current}).");
        return Ok(());
    }

    println!("New version available: {latest} (current: v{current})");

    let arch = detect_arch_suffix();
    let asset_name = format!("opcs-{arch}");
    let download_url = format!("https://github.com/{REPO}/releases/download/{latest}/{asset_name}");

    println!("Downloading {asset_name}...");

    let tmp = std::env::temp_dir().join("opcs-upgrade");
    let status = Command::new("curl")
        .args(["-fsSL", &download_url, "-o", tmp.to_str().unwrap()])
        .status()
        .context("failed to run curl")?;

    if !status.success() {
        anyhow::bail!("download failed (HTTP error)");
    }

    std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o755))
        .context("failed to set executable permission")?;

    std::fs::rename(&tmp, &current_exe)
        .or_else(|_| {
            // Fallback: try with sudo if we don't have write permission
            let status = Command::new("sudo")
                .args(["cp", tmp.to_str().unwrap(), current_exe.to_str().unwrap()])
                .status()
                .context("failed to copy with sudo")?;
            std::fs::remove_file(&tmp).ok();
            if status.success() {
                Ok(())
            } else {
                anyhow::bail!("failed to replace binary (try with sudo manually)")
            }
        })
        .context("failed to replace binary")?;

    println!("✓ Updated to {latest}");
    Ok(())
}

fn latest_release_tag() -> Result<String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let output = Command::new("curl")
        .args(["-fsSL", &url])
        .output()
        .context("failed to fetch latest release info")?;

    if !output.status.success() {
        anyhow::bail!("failed to reach GitHub API");
    }

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("failed to parse release JSON")?;
    let tag = json["tag_name"]
        .as_str()
        .context("missing tag_name in release response")?;
    Ok(tag.to_string())
}

fn detect_arch_suffix() -> &'static str {
    let arch = std::env::consts::ARCH;
    match arch {
        "x86_64" => "x86_64-linux",
        "aarch64" => "aarch64-linux",
        _ => "x86_64-linux",
    }
}
