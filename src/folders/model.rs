/// A folder used to organize sessions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

impl Folder {
    /// Create a new folder with the given ID and name.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            parent_id: None,
        }
    }
}

/// Maps a session to a folder.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FolderMapping {
    pub session_id: String,
    pub folder_id: String,
}

impl FolderMapping {
    pub fn new(session_id: impl Into<String>, folder_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            folder_id: folder_id.into(),
        }
    }
}
