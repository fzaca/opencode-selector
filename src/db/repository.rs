use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension};

use super::models::{Project, Session};

/// Repository for reading and writing opencode session data.
pub struct SessionRepository {
    conn: Connection,
}

impl SessionRepository {
    /// Open a connection to the opencode SQLite database.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path).context("failed to open opencode database")?;
        Ok(Self { conn })
    }

    /// Open an in-memory connection (useful for tests).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("failed to open in-memory database")?;
        Ok(Self { conn })
    }

    /// Load all projects keyed by ID.
    pub fn projects(&self) -> Result<HashMap<String, Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM project")
            .context("failed to prepare project query")?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .context("failed to query projects")?;

        let mut projects = HashMap::new();
        for row in rows {
            let project = row.context("failed to read project row")?;
            projects.insert(project.id.clone(), project);
        }
        Ok(projects)
    }

    /// Load project directories.
    pub fn project_directories(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT project_id, directory FROM project_directory")
            .context("failed to prepare project_directory query")?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .context("failed to query project directories")?;

        let mut dirs = Vec::new();
        for row in rows {
            dirs.push(row.context("failed to read project_directory row")?);
        }
        Ok(dirs)
    }

    /// Load project directories keyed by project ID.
    fn project_directory_map(&self) -> Result<HashMap<String, String>> {
        let mut map = HashMap::new();
        for (project_id, directory) in self.project_directories()? {
            map.insert(project_id, directory);
        }
        Ok(map)
    }

    /// Find the project ID whose directory matches or contains the given path.
    pub fn find_project_for_path(&self, path: impl AsRef<Path>) -> Result<Option<String>> {
        let target = std::fs::canonicalize(path.as_ref())
            .with_context(|| format!("failed to canonicalize {}", path.as_ref().display()))?;
        let dirs = self.project_directories()?;
        let mut best_match: Option<(String, usize)> = None;
        for (project_id, dir) in dirs {
            let project_path = Path::new(&dir);
            let canonical = match std::fs::canonicalize(project_path) {
                Ok(p) => p,
                Err(_) => continue,
            };
            if target == canonical || target.starts_with(&canonical) {
                let depth = canonical.components().count();
                if best_match.as_ref().map(|(_, d)| depth > *d).unwrap_or(true) {
                    best_match = Some((project_id, depth));
                }
            }
        }
        Ok(best_match.map(|(id, _)| id))
    }

    /// List sessions ordered by most recently updated.
    pub fn list_sessions(&self) -> Result<Vec<Session>> {
        let projects = self.projects()?;
        let project_dirs = self.project_directory_map()?;
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                    id, project_id, slug, title, agent, model,
                    time_created, time_updated, summary_files
                 FROM session
                 ORDER BY time_updated DESC",
            )
            .context("failed to prepare session query")?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let project_id: String = row.get(1)?;
                let project_name = projects
                    .get(&project_id)
                    .and_then(|p| p.name.clone())
                    .unwrap_or_default();
                let project_directory = project_dirs.get(&project_id).cloned();
                let first_message_preview =
                    Self::first_message_preview(&self.conn, &id).ok().flatten();
                let messages =
                    Self::session_messages(&self.conn, &id).unwrap_or_default();

                Ok(Session {
                    id,
                    project_id,
                    project_name,
                    project_directory,
                    slug: row.get(2)?,
                    title: row.get(3)?,
                    agent: row.get(4)?,
                    model_name: parse_model_name(row.get(5)?),
                    created_at: parse_time(row.get(6)?),
                    updated_at: parse_time(row.get(7)?),
                    summary_files: row.get(8)?,
                    first_message_preview,
                    messages,
                })
            })
            .context("failed to query sessions")?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row.context("failed to read session row")?);
        }
        Ok(sessions)
    }

    /// Get a single session by ID.
    pub fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let projects = self.projects()?;
        let project_dirs = self.project_directory_map()?;
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                    id, project_id, slug, title, agent, model,
                    time_created, time_updated, summary_files
                 FROM session
                 WHERE id = ?1",
            )
            .context("failed to prepare session lookup")?;

        let session = stmt
            .query_row([id], |row| {
                let id: String = row.get(0)?;
                let project_id: String = row.get(1)?;
                let project_name = projects
                    .get(&project_id)
                    .and_then(|p| p.name.clone())
                    .unwrap_or_default();
                let project_directory = project_dirs.get(&project_id).cloned();
                let first_message_preview =
                    Self::first_message_preview(&self.conn, &id).ok().flatten();
                let messages =
                    Self::session_messages(&self.conn, &id).unwrap_or_default();

                Ok(Session {
                    id,
                    project_id,
                    project_name,
                    project_directory,
                    slug: row.get(2)?,
                    title: row.get(3)?,
                    agent: row.get(4)?,
                    model_name: parse_model_name(row.get(5)?),
                    created_at: parse_time(row.get(6)?),
                    updated_at: parse_time(row.get(7)?),
                    summary_files: row.get(8)?,
                    first_message_preview,
                    messages,
                })
            })
            .optional()
            .context("failed to lookup session")?;
        Ok(session)
    }

    /// Rename a session title.
    pub fn rename_session(&self, id: &str, title: &str) -> Result<()> {
        self.conn
            .execute("UPDATE session SET title = ?1 WHERE id = ?2", [title, id])
            .context("failed to rename session")?;
        Ok(())
    }

    /// Permanently delete a session and its associated rows.
    pub fn delete_session(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM message WHERE session_id = ?1", [id])
            .context("failed to delete session messages")?;
        self.conn
            .execute("DELETE FROM part WHERE session_id = ?1", [id])
            .context("failed to delete session parts")?;
        self.conn
            .execute("DELETE FROM todo WHERE session_id = ?1", [id])
            .context("failed to delete session todos")?;
        self.conn
            .execute("DELETE FROM session WHERE id = ?1", [id])
            .context("failed to delete session")?;
        Ok(())
    }

    /// Get the preview of the first user message in a session.
    fn first_message_preview(conn: &Connection, session_id: &str) -> Result<Option<String>> {
        let mut stmt = conn
            .prepare(
                "SELECT data FROM part
                 WHERE session_id = ?1
                 ORDER BY time_created ASC
                 LIMIT 50",
            )
            .context("failed to prepare part query")?;

        let rows = stmt
            .query_map([session_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .context("failed to query parts")?;

        for row in rows {
            let data = row.context("failed to read part row")?;
            if let Some(text) = extract_text_preview(&data) {
                return Ok(Some(text));
            }
        }
        Ok(None)
    }

    pub fn session_messages(conn: &Connection, session_id: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = conn
            .prepare(
                "SELECT data FROM part
                 WHERE session_id = ?1
                 ORDER BY time_created ASC",
            )
            .context("failed to prepare part query")?;

        let rows = stmt
            .query_map([session_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .context("failed to query parts")?;

        let mut messages = Vec::new();
        for row in rows {
            let data = row.context("failed to read part row")?;
            let value: serde_json::Value = match serde_json::from_str(&data) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let role = value
                .get("role")
                .and_then(|r| r.as_str())
                .unwrap_or("unknown")
                .to_string();
            let text = value
                .get("text")
                .or_else(|| value.get("content"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            if !text.is_empty() {
                messages.push((role, text.to_string()));
            }
        }
        Ok(messages)
    }
}

fn parse_time(timestamp_ms: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(timestamp_ms).unwrap_or_else(Utc::now)
}

fn parse_model_name(raw: Option<String>) -> Option<String> {
    let raw = raw?;
    if raw.is_empty() {
        return None;
    }
    // The model column is JSON like {"id":"...","providerID":"...","variant":"..."}.
    // Try to extract the id field.
    serde_json::from_str::<serde_json::Value>(&raw)
        .ok()
        .and_then(|v| v.get("id").and_then(|id| id.as_str().map(String::from)))
        .or(Some(raw))
}

fn extract_text_preview(data: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(data).ok()?;
    // Try common shapes for the first user message.
    if let Some(text) = value.get("text").and_then(|t| t.as_str()) {
        return Some(truncate(text));
    }
    if let Some(content) = value.get("content").and_then(|c| c.as_str()) {
        return Some(truncate(content));
    }
    None
}

fn truncate(text: &str) -> String {
    const MAX_LEN: usize = 300;
    if text.len() <= MAX_LEN {
        text.to_string()
    } else {
        format!("{}...", &text[..MAX_LEN])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_schema(repo: &SessionRepository) {
        repo.conn
            .execute(
                "CREATE TABLE project (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "CREATE TABLE session (
                    id TEXT PRIMARY KEY,
                    project_id TEXT NOT NULL,
                    slug TEXT NOT NULL,
                    title TEXT,
                    agent TEXT,
                    model TEXT,
                    time_created INTEGER,
                    time_updated INTEGER,
                    summary_files INTEGER
                )",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "CREATE TABLE message (
                    id TEXT PRIMARY KEY,
                    session_id TEXT NOT NULL,
                    time_created INTEGER,
                    data TEXT
                )",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "CREATE TABLE part (
                    id TEXT PRIMARY KEY,
                    message_id TEXT NOT NULL,
                    session_id TEXT NOT NULL,
                    time_created INTEGER,
                    data TEXT
                )",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "CREATE TABLE todo (
                    session_id TEXT NOT NULL,
                    content TEXT,
                    status TEXT,
                    priority TEXT,
                    position INTEGER,
                    time_created INTEGER,
                    time_updated INTEGER
                )",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "CREATE TABLE project_directory (
                    project_id TEXT NOT NULL,
                    directory TEXT NOT NULL,
                    branch TEXT,
                    time_created INTEGER
                )",
                [],
            )
            .unwrap();
    }

    #[test]
    fn list_sessions_orders_by_updated_desc() {
        let repo = SessionRepository::open_in_memory().unwrap();
        setup_schema(&repo);

        repo.conn
            .execute(
                "INSERT INTO project (id, name) VALUES ('p1', 'project-one')",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "INSERT INTO session
                 (id, project_id, slug, title, agent, model, time_created, time_updated, summary_files)
                 VALUES
                 ('s1', 'p1', 'alpha', 'Alpha', 'explore', '{\"id\":\"m1\"}', 1000, 3000, 0),
                 ('s2', 'p1', 'beta', 'Beta', NULL, NULL, 1000, 5000, 2)",
                [],
            )
            .unwrap();

        let sessions = repo.list_sessions().unwrap();
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].id, "s2");
        assert_eq!(sessions[0].model_name, None);
        assert_eq!(sessions[1].id, "s1");
        assert_eq!(sessions[1].model_name, Some("m1".to_string()));
    }

    #[test]
    fn get_session_returns_none_for_unknown() {
        let repo = SessionRepository::open_in_memory().unwrap();
        setup_schema(&repo);
        assert!(repo.get_session("unknown").unwrap().is_none());
    }

    #[test]
    fn rename_session_updates_title() {
        let repo = SessionRepository::open_in_memory().unwrap();
        setup_schema(&repo);
        repo.conn
            .execute(
                "INSERT INTO project (id, name) VALUES ('p1', 'project-one')",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "INSERT INTO session
                 (id, project_id, slug, title, time_created, time_updated, summary_files)
                 VALUES ('s1', 'p1', 'alpha', 'Old', 1000, 1000, 0)",
                [],
            )
            .unwrap();

        repo.rename_session("s1", "New").unwrap();
        let session = repo.get_session("s1").unwrap().unwrap();
        assert_eq!(session.title, "New");
    }

    #[test]
    fn delete_session_removes_rows() {
        let repo = SessionRepository::open_in_memory().unwrap();
        setup_schema(&repo);
        repo.conn
            .execute(
                "INSERT INTO project (id, name) VALUES ('p1', 'project-one')",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "INSERT INTO session
                 (id, project_id, slug, title, time_created, time_updated, summary_files)
                 VALUES ('s1', 'p1', 'alpha', 'Old', 1000, 1000, 0)",
                [],
            )
            .unwrap();
        repo.conn
            .execute(
                "INSERT INTO message (id, session_id, time_created, data)
                 VALUES ('m1', 's1', 1000, '{}')",
                [],
            )
            .unwrap();

        repo.delete_session("s1").unwrap();
        assert!(repo.get_session("s1").unwrap().is_none());
        let count: i64 = repo
            .conn
            .query_row(
                "SELECT COUNT(*) FROM message WHERE session_id = 's1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }
}
