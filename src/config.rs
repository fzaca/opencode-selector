use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;

/// User-facing configuration loaded from `~/.config/opencode-selector/config.toml`.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigFile {
    /// Whether the folder system is enabled in the TUI.
    #[serde(default)]
    pub folders_enabled: bool,

    /// Optional color overrides. When unset the app falls back to the
    /// terminal's default 16-color palette.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<ThemeConfig>,
}

/// Optional color theme overrides in `config.toml`.
///
/// Colors may be given as `#RRGGBB` or `#RGB` hex strings. Any field that is
/// omitted uses the terminal-adaptive default.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_dim: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight_dim: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<String>,
}

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
    /// Path to the user config file.
    config_file_path: PathBuf,
    /// Parsed user config.
    config_file: ConfigFile,
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
        let config_file_path = selector_config_dir.join("config.toml");

        let config_file = Self::load_config_file(&config_file_path);

        Ok(Self {
            selector_config_dir,
            selector_data_dir,
            opencode_db_path,
            opencode_config_dir,
            folders_path,
            config_file_path,
            config_file,
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

    /// Override the config file path (useful for tests).
    pub fn with_config_file_path(mut self, path: impl AsRef<Path>) -> Self {
        self.config_file_path = path.as_ref().to_path_buf();
        self.config_file = Self::load_config_file(&self.config_file_path);
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

    pub fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }

    pub fn folders_enabled(&self) -> bool {
        self.config_file.folders_enabled
    }

    pub fn theme(&self) -> Option<&ThemeConfig> {
        self.config_file.theme.as_ref()
    }

    /// Enable or disable folders in memory.
    pub fn set_folders_enabled(&mut self, enabled: bool) {
        self.config_file.folders_enabled = enabled;
    }

    /// Persist the current config file to disk.
    pub fn save_config_file(&self) -> Result<()> {
        if let Some(parent) = self.config_file_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory {}", parent.display()))?;
        }
        let content =
            toml::to_string_pretty(&self.config_file).context("failed to serialize config file")?;
        std::fs::write(&self.config_file_path, content)
            .with_context(|| format!("failed to write {}", self.config_file_path.display()))?;
        Ok(())
    }

    fn load_config_file(path: &Path) -> ConfigFile {
        if !path.exists() {
            return ConfigFile::default();
        }
        match std::fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => ConfigFile::default(),
        }
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

    #[test]
    fn folders_disabled_by_default() {
        let config = Config::new().unwrap();
        assert!(!config.folders_enabled());
    }
}
