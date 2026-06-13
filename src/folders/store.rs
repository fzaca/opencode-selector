use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::model::{Folder, FolderMapping};

/// On-disk representation of the folders sidecar file.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
struct FolderFile {
    #[serde(default)]
    folders: Vec<Folder>,
    #[serde(default)]
    mappings: Vec<FolderMapping>,
}

/// Manages folder metadata stored outside of opencode's database.
pub struct FolderStore {
    path: PathBuf,
    data: FolderFile,
}

impl FolderStore {
    /// Open or create a folder store at the given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read folder store {}", path.display()))?;
            let data: FolderFile = toml::from_str(&content)
                .with_context(|| format!("failed to parse folder store {}", path.display()))?;
            let mut store = Self { path, data };
            store.ensure_defaults();
            Ok(store)
        } else {
            let mut store = Self {
                path,
                data: FolderFile::default(),
            };
            store.ensure_defaults();
            store.save()?;
            Ok(store)
        }
    }

    /// Create the store in memory without touching disk.
    pub fn in_memory() -> Self {
        let mut store = Self {
            path: PathBuf::new(),
            data: FolderFile::default(),
        };
        store.ensure_defaults();
        store
    }

    /// Persist the current state to disk.
    pub fn save(&self) -> Result<()> {
        if self.path.as_os_str().is_empty() {
            return Ok(());
        }
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory {}", parent.display()))?;
        }
        let content =
            toml::to_string_pretty(&self.data).context("failed to serialize folder store")?;
        std::fs::write(&self.path, content)
            .with_context(|| format!("failed to write folder store {}", self.path.display()))?;
        Ok(())
    }

    /// List all folders.
    pub fn folders(&self) -> &[Folder] {
        &self.data.folders
    }

    /// Find a folder by ID.
    pub fn folder(&self, id: &str) -> Option<&Folder> {
        self.data.folders.iter().find(|f| f.id == id)
    }

    /// Add a new folder.
    pub fn add_folder(&mut self, folder: Folder) -> Result<()> {
        if self.data.folders.iter().any(|f| f.id == folder.id) {
            anyhow::bail!("folder with id {} already exists", folder.id);
        }
        self.data.folders.push(folder);
        self.save()
    }

    /// Rename a folder.
    pub fn rename_folder(&mut self, id: &str, name: &str) -> Result<()> {
        if Self::is_system_folder(id) {
            anyhow::bail!("cannot rename system folder {}", id);
        }
        let folder = self
            .data
            .folders
            .iter_mut()
            .find(|f| f.id == id)
            .with_context(|| format!("folder {} not found", id))?;
        folder.name = name.to_string();
        self.save()
    }

    /// Delete a folder and all mappings pointing to it.
    pub fn delete_folder(&mut self, id: &str) -> Result<()> {
        if Self::is_system_folder(id) {
            anyhow::bail!("cannot delete system folder {}", id);
        }
        let original_len = self.data.folders.len();
        self.data.folders.retain(|f| f.id != id);
        if self.data.folders.len() == original_len {
            anyhow::bail!("folder {} not found", id);
        }
        self.data.mappings.retain(|m| m.folder_id != id);
        self.save()
    }

    fn is_system_folder(id: &str) -> bool {
        matches!(id, "all" | "inbox" | "archive")
    }

    /// Move a session into a folder.
    pub fn move_session(&mut self, session_id: &str, folder_id: &str) -> Result<()> {
        if folder_id == "all" {
            anyhow::bail!("cannot move sessions to the All folder");
        }
        if self.folder(folder_id).is_none() {
            anyhow::bail!("folder {} not found", folder_id);
        }
        self.data.mappings.retain(|m| m.session_id != session_id);
        self.data
            .mappings
            .push(FolderMapping::new(session_id, folder_id));
        self.save()
    }

    /// Remove a session from any folder.
    pub fn unassign_session(&mut self, session_id: &str) -> Result<()> {
        self.data.mappings.retain(|m| m.session_id != session_id);
        self.save()
    }

    /// Return the folder ID for a session, if any.
    pub fn folder_for_session(&self, session_id: &str) -> Option<&str> {
        self.data
            .mappings
            .iter()
            .find(|m| m.session_id == session_id)
            .map(|m| m.folder_id.as_str())
    }

    /// Return all session IDs assigned to a folder.
    pub fn sessions_in_folder(&self, folder_id: &str) -> Vec<&str> {
        self.data
            .mappings
            .iter()
            .filter(|m| m.folder_id == folder_id)
            .map(|m| m.session_id.as_str())
            .collect()
    }

    /// Build a mapping from session ID to folder ID for quick lookup.
    pub fn session_folder_map(&self) -> HashMap<String, String> {
        self.data
            .mappings
            .iter()
            .map(|m| (m.session_id.clone(), m.folder_id.clone()))
            .collect()
    }

    /// Return true if the folder has no children folders and no sessions.
    pub fn is_empty(&self, folder_id: &str) -> bool {
        let has_children = self
            .data
            .folders
            .iter()
            .any(|f| f.parent_id.as_deref() == Some(folder_id));
        let has_sessions = self.sessions_in_folder(folder_id).is_empty();
        !has_children && has_sessions
    }

    fn ensure_defaults(&mut self) {
        let default_ids: &[&str] = &["all", "inbox", "archive"];
        for id in default_ids {
            if !self.data.folders.iter().any(|f| f.id == *id) {
                let name = match *id {
                    "all" => "All",
                    "inbox" => "Inbox",
                    "archive" => "Archive",
                    _ => id,
                };
                self.data.folders.push(Folder::new(*id, name));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn store_creates_defaults() {
        let store = FolderStore::in_memory();
        let folders: Vec<_> = store.folders().to_vec();
        assert_eq!(folders.len(), 3);
        assert_eq!(folders[0].id, "all");
        assert_eq!(folders[1].id, "inbox");
        assert_eq!(folders[2].id, "archive");
    }

    #[test]
    fn move_and_lookup_session() {
        let mut store = FolderStore::in_memory();
        store.add_folder(Folder::new("work", "Work")).unwrap();
        store.move_session("s1", "work").unwrap();
        assert_eq!(store.folder_for_session("s1"), Some("work"));
    }

    #[test]
    fn persistence_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        {
            let mut store = FolderStore::open(tmp.path()).unwrap();
            store
                .add_folder(Folder::new("personal", "Personal"))
                .unwrap();
            store.move_session("s1", "personal").unwrap();
        }
        let store = FolderStore::open(tmp.path()).unwrap();
        assert!(store.folder("personal").is_some());
        assert_eq!(store.folder_for_session("s1"), Some("personal"));
    }
}
