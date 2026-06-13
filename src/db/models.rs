use chrono::{DateTime, Utc};

/// Metadata about an opencode session.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub project_id: String,
    pub project_name: String,
    pub agent: Option<String>,
    pub model_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub summary_files: i64,
    pub first_message_preview: Option<String>,
}

impl Session {
    /// Return a short display title, falling back to the slug.
    pub fn display_title(&self) -> &str {
        if self.title.is_empty() {
            &self.slug
        } else {
            &self.title
        }
    }
}

/// Metadata about an opencode project.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: String,
    pub name: Option<String>,
}
