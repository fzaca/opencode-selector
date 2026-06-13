use std::collections::HashMap;

use crate::db::Session;
use crate::folders::Folder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Updated,
    Created,
    Title,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    Preview,
    Help,
    ConfirmDelete,
    Rename,
    MoveToFolder,
    NewFolder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
    Rename,
    NewFolder,
    Confirm,
}

#[derive(Debug)]
pub struct App {
    pub sessions: Vec<Session>,
    pub folders: Vec<Folder>,
    pub session_folder_map: HashMap<String, String>,
    pub filtered_indices: Vec<usize>,
    pub selected_session: usize,
    pub selected_folder: usize,
    pub sort_by: SortBy,
    pub screen: Screen,
    pub input_mode: InputMode,
    pub search_query: String,
    pub rename_buffer: String,
    pub new_folder_buffer: String,
    pub confirm_message: String,
    pub pending_delete_id: Option<String>,
    pub status_message: Option<String>,
    pub pending_key: Option<char>,
    pub project_filter: Option<String>,
    pub current_project: Option<String>,
    pub folders_enabled: bool,
    pub global_mode: bool,
}

impl App {
    pub fn new(
        sessions: Vec<Session>,
        folders: Vec<Folder>,
        mappings: HashMap<String, String>,
        project_filter: Option<String>,
        folders_enabled: bool,
        global_mode: bool,
    ) -> Self {
        let current_project = project_filter.clone();
        let mut app = Self {
            sessions,
            folders,
            session_folder_map: mappings,
            filtered_indices: Vec::new(),
            selected_session: 0,
            selected_folder: 0,
            sort_by: SortBy::Updated,
            screen: Screen::Main,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            rename_buffer: String::new(),
            new_folder_buffer: String::new(),
            confirm_message: String::new(),
            pending_delete_id: None,
            status_message: None,
            pending_key: None,
            project_filter,
            current_project,
            folders_enabled,
            global_mode,
        };
        app.apply_filter_and_sort();
        app
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.filtered_indices
            .get(self.selected_session)
            .and_then(|&idx| self.sessions.get(idx))
    }

    pub fn current_folder(&self) -> Option<&Folder> {
        self.folders.get(self.selected_folder)
    }

    pub fn apply_filter_and_sort(&mut self) {
        let folder_id = self.current_folder().map(|f| f.id.as_str());

        let mut matches: Vec<usize> = self
            .sessions
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                // Filter by current project if one is set.
                if let Some(pid) = self.project_filter.as_ref() {
                    if &s.project_id != pid {
                        return false;
                    }
                }
                // The All folder shows every session. Every other folder shows
                // only sessions explicitly assigned to it.
                if let Some(fid) = folder_id {
                    if fid == "all" {
                        true
                    } else {
                        self.session_folder_map
                            .get(&s.id)
                            .map(|f| f == fid)
                            .unwrap_or(false)
                    }
                } else {
                    true
                }
            })
            .filter(|(_, s)| {
                if self.search_query.is_empty() {
                    return true;
                }
                let q = self.search_query.to_lowercase();
                s.display_title().to_lowercase().contains(&q)
                    || s.slug.to_lowercase().contains(&q)
                    || s.project_name.to_lowercase().contains(&q)
                    || s.first_message_preview
                        .as_ref()
                        .map(|m| m.to_lowercase().contains(&q))
                        .unwrap_or(false)
            })
            .map(|(idx, _)| idx)
            .collect();

        match self.sort_by {
            SortBy::Updated => matches.sort_by_key(|&idx| {
                let s = &self.sessions[idx];
                (std::cmp::Reverse(s.updated_at), s.title.clone())
            }),
            SortBy::Created => matches.sort_by_key(|&idx| {
                let s = &self.sessions[idx];
                (std::cmp::Reverse(s.created_at), s.title.clone())
            }),
            SortBy::Title => matches.sort_by_key(|&idx| self.sessions[idx].title.clone()),
        }

        self.filtered_indices = matches;
        if self.selected_session >= self.filtered_indices.len() {
            self.selected_session = self.filtered_indices.len().saturating_sub(1);
        }
    }

    pub fn move_selection_down(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected_session =
                (self.selected_session + 1).min(self.filtered_indices.len() - 1);
        }
    }

    pub fn move_selection_up(&mut self) {
        self.selected_session = self.selected_session.saturating_sub(1);
    }

    pub fn move_folder_down(&mut self) {
        if !self.folders.is_empty() {
            self.selected_folder = (self.selected_folder + 1).min(self.folders.len() - 1);
            self.apply_filter_and_sort();
        }
    }

    pub fn move_folder_up(&mut self) {
        self.selected_folder = self.selected_folder.saturating_sub(1);
        self.apply_filter_and_sort();
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
        self.selected_session = 0;
        self.apply_filter_and_sort();
    }

    pub fn cycle_sort(&mut self) {
        self.sort_by = match self.sort_by {
            SortBy::Updated => SortBy::Created,
            SortBy::Created => SortBy::Title,
            SortBy::Title => SortBy::Updated,
        };
        self.apply_filter_and_sort();
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn toggle_project_filter(&mut self) {
        if self.project_filter.is_some() {
            self.project_filter = None;
            self.set_status("Showing all projects");
        } else {
            self.project_filter = self.current_project.clone();
            if let Some(ref pid) = self.project_filter {
                self.set_status(format!("Filtered to project {pid}"));
            }
        }
        self.apply_filter_and_sort();
    }

    pub fn set_project_filter(&mut self, project_id: Option<String>) {
        self.project_filter = project_id;
        self.apply_filter_and_sort();
    }

    pub fn toggle_folders(&mut self) {
        self.folders_enabled = !self.folders_enabled;
        if self.folders_enabled {
            self.set_status("Folders enabled");
        } else {
            self.set_status("Folders disabled");
        }
    }
}
