use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;

/// Application configuration and path resolution.
#[derive(Debug, Clone)]
pub struct Config {
    /// Directory for opencode-selector's own configuration.
    selector_config_dir: PathBuf,
    /// Directory for opencode-selector's data files.
    selector_data_dir: PathBuf,
    /// Path to opencode's SQLite database.
    opencode_db_path: PathBuf,
    /// Path to opencode's configuration directory.
    opencode_config_dir: PathBuf,
    /// Path to the folder sidecar file.
    folders_path: PathBuf,
}

impl Config {
    /// Build the default configuration using standard directories.
    pub fn new() -> Result<Self> {
        let selector_dirs = ProjectDirs::from("", "", "opencode-selector")
            .context("failed to determine opencode-selector project directories")?;
        let selector_config_dir = selector_dirs.config_dir().to_path_buf();
        let selector_data_dir = selector_dirs.data_dir().to_path_buf();

        let home_dir = directories::UserDirs::new()
            .context("failed to determine user home directory")?
            .home_dir()
            .to_path_buf();

        let opencode_data_dir = home_dir.join(".local/share/opencode");
        let opencode_config_dir = home_dir.join(".config/opencode");
        let opencode_db_path = opencode_data_dir.join("opencode.db");
        let folders_path = selector_config_dir.join("folders.toml");

        Ok(Self {
            selector_config_dir,
            selector_data_dir,
            opencode_db_path,
            opencode_config_dir,
            folders_path,
        })
    }

    /// Override the opencode database path (useful for tests).
    pub fn with_opencode_db_path(mut self, path: impl AsRef<Path>) -> Self {
        self.opencode_db_path = path.as_ref().to_path_buf();
        self
    }

    /// Override the folders sidecar path (useful for tests).
    pub fn with_folders_path(mut self, path: impl AsRef<Path>) -> Self {
        self.folders_path = path.as_ref().to_path_buf();
        self
    }

    pub fn selector_config_dir(&self) -> &Path {
        &self.selector_config_dir
    }

    pub fn selector_data_dir(&self) -> &Path {
        &self.selector_data_dir
    }

    pub fn opencode_db_path(&self) -> &Path {
        &self.opencode_db_path
    }

    pub fn opencode_config_dir(&self) -> &Path {
        &self.opencode_config_dir
    }

    pub fn folders_path(&self) -> &Path {
        &self.folders_path
    }

    /// Ensure that the selector configuration and data directories exist.
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.selector_config_dir).with_context(|| {
            format!(
                "failed to create config directory {}",
                self.selector_config_dir.display()
            )
        })?;
        std::fs::create_dir_all(&self.selector_data_dir).with_context(|| {
            format!(
                "failed to create data directory {}",
                self.selector_data_dir.display()
            )
        })?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("failed to build default config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_paths_are_resolved() {
        let config = Config::new().unwrap();
        assert!(
            config
                .selector_config_dir()
                .to_string_lossy()
                .contains("opencode-selector")
        );
        assert!(
            config
                .opencode_db_path()
                .to_string_lossy()
                .contains("opencode.db")
        );
    }
}
