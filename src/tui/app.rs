use std::collections::HashMap;

use crate::db::Session;
use crate::folders::Folder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    Executed,
    Unknown(String),
    Error(String),
    Quit,
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub args: &'static [ArgSpec],
}

#[derive(Debug, Clone)]
pub struct ArgSpec {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub description: &'static str,
}

pub static COMMANDS: &[CommandSpec] = &[
    CommandSpec {
        name: "quit",
        aliases: &["q", "exit"],
        description: "Exit the application",
        args: &[],
    },
    CommandSpec {
        name: "sort",
        aliases: &[],
        description: "Change session sort order",
        args: &[ArgSpec {
            name: "order",
            values: &["updated", "created", "title"],
        }],
    },
    CommandSpec {
        name: "filter",
        aliases: &["f"],
        description: "Filter sessions by project",
        args: &[ArgSpec {
            name: "project",
            values: &[],
        }],
    },
    CommandSpec {
        name: "help",
        aliases: &["?"],
        description: "Show help screen",
        args: &[],
    },
    CommandSpec {
        name: "search",
        aliases: &["/"],
        description: "Search sessions",
        args: &[ArgSpec {
            name: "query",
            values: &[],
        }],
    },
    CommandSpec {
        name: "folders",
        aliases: &["fs"],
        description: "Toggle or set folders panel",
        args: &[ArgSpec {
            name: "state",
            values: &["on", "off", "toggle"],
        }],
    },
    CommandSpec {
        name: "goto",
        aliases: &["g"],
        description: "Go to session by number or position",
        args: &[ArgSpec {
            name: "target",
            values: &["top", "first", "bottom", "last"],
        }],
    },
    CommandSpec {
        name: "global",
        aliases: &["G"],
        description: "Show all projects (global mode)",
        args: &[],
    },
    CommandSpec {
        name: "cwd",
        aliases: &[],
        description: "Show only current project (CWD mode)",
        args: &[],
    },
    CommandSpec {
        name: "clear",
        aliases: &["cls"],
        description: "Clear search query",
        args: &[],
    },
];

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
    Command,
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

    pub command_buffer: String,
    pub command_history: Vec<String>,
    pub command_history_pos: Option<usize>,
    pub command_suggestions: Vec<Suggestion>,
    pub selected_suggestion: usize,

    pub preview_scroll: u16,
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

            command_buffer: String::new(),
            command_history: Vec::new(),
            command_history_pos: None,
            command_suggestions: Vec::new(),
            selected_suggestion: 0,

            preview_scroll: 0,
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
                    if s.project_id != *pid {
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

    pub fn compute_suggestions(&mut self) {
        self.command_suggestions.clear();
        let buffer = self.command_buffer.to_lowercase();
        if buffer.is_empty() {
            for cmd in COMMANDS {
                self.command_suggestions.push(Suggestion {
                    text: cmd.name.to_string(),
                    description: cmd.description,
                });
            }
            self.selected_suggestion = 0;
            return;
        }

        let parts: Vec<&str> = buffer.split_whitespace().collect();
        let cmd_part = parts[0];

        let mut scored: Vec<(isize, Suggestion)> = Vec::new();

        if parts.len() <= 1 {
            let candidates: Vec<&str> = COMMANDS
                .iter()
                .flat_map(|c| {
                    let mut names = vec![c.name];
                    names.extend(c.aliases);
                    names
                })
                .collect();

            for candidate in &candidates {
                if let Some(score) = fuzzy_score(cmd_part, candidate) {
                    scored.push((
                        score,
                        Suggestion {
                            text: candidate.to_string(),
                            description: COMMANDS
                                .iter()
                                .find(|c| c.name == *candidate || c.aliases.contains(candidate))
                                .map(|c| c.description)
                                .unwrap_or(""),
                        },
                    ));
                }
            }
        } else {
            let matched = COMMANDS
                .iter()
                .find(|c| c.name == cmd_part || c.aliases.contains(&cmd_part));
            if let Some(cmd) = matched {
                if let Some(arg) = cmd.args.first() {
                    if !arg.values.is_empty() {
                        let arg_part = parts[1..].join(" ").to_lowercase();
                        for value in arg.values {
                            if let Some(score) = fuzzy_score(&arg_part, value) {
                                scored.push((
                                    score,
                                    Suggestion {
                                        text: format!("{} {}", cmd.name, value),
                                        description: "",
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        self.command_suggestions = scored.into_iter().map(|(_, s)| s).collect();
        if self.selected_suggestion >= self.command_suggestions.len() {
            self.selected_suggestion = self.command_suggestions.len().saturating_sub(1);
        }
    }

    pub fn accept_suggestion(&mut self) {
        if self.command_suggestions.is_empty() {
            return;
        }
        let sel = self
            .selected_suggestion
            .min(self.command_suggestions.len() - 1);
        self.command_buffer = self.command_suggestions[sel].text.clone();
        self.command_suggestions.clear();
        self.selected_suggestion = 0;
    }

    pub fn toggle_folders(&mut self) {
        self.folders_enabled = !self.folders_enabled;
        if self.folders_enabled {
            self.set_status("Folders enabled");
        } else {
            self.set_status("Folders disabled");
        }
    }

    pub fn execute_command(&mut self, input: &str) -> CommandResult {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return CommandResult::Executed;
        }

        self.command_history.push(trimmed.to_string());
        self.command_history_pos = None;

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        match parts[0] {
            "q" | "quit" | "exit" => CommandResult::Quit,
            "sort" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "updated" => self.sort_by = SortBy::Updated,
                        "created" => self.sort_by = SortBy::Created,
                        "title" => self.sort_by = SortBy::Title,
                        other => return CommandResult::Error(format!("Unknown sort: {other}")),
                    }
                    self.apply_filter_and_sort();
                    self.set_status(format!("Sorted by {}", sort_label(self.sort_by)));
                } else {
                    self.cycle_sort();
                }
                CommandResult::Executed
            }
            "filter" | "f" => {
                if parts.len() > 1 {
                    let project = parts[1..].join(" ");
                    self.project_filter = Some(project.clone());
                    self.apply_filter_and_sort();
                    self.set_status(format!("Filtered to project '{project}'"));
                } else {
                    self.project_filter = None;
                    self.apply_filter_and_sort();
                    self.set_status("Showing all projects");
                }
                CommandResult::Executed
            }
            "help" | "?" => {
                self.screen = Screen::Help;
                CommandResult::Executed
            }
            "search" | "/" => {
                let query = if parts.len() > 1 {
                    parts[1..].join(" ")
                } else {
                    String::new()
                };
                self.search_query = query;
                self.apply_filter_and_sort();
                CommandResult::Executed
            }
            "folders" | "fs" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "on" | "enable" => {
                            self.folders_enabled = true;
                            self.set_status("Folders enabled");
                        }
                        "off" | "disable" => {
                            self.folders_enabled = false;
                            self.set_status("Folders disabled");
                        }
                        "toggle" | "t" => self.toggle_folders(),
                        other => return CommandResult::Error(format!("Unknown: {other}")),
                    }
                } else {
                    self.toggle_folders();
                }
                CommandResult::Executed
            }
            "goto" | "g" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "top" | "first" => self.selected_session = 0,
                        "bottom" | "last" => {
                            self.selected_session = self.filtered_indices.len().saturating_sub(1);
                        }
                        num => {
                            if let Ok(n) = num.parse::<usize>() {
                                let idx = n.saturating_sub(1);
                                self.selected_session =
                                    idx.min(self.filtered_indices.len().saturating_sub(1));
                            } else {
                                return CommandResult::Error(format!("Invalid number: {num}"));
                            }
                        }
                    }
                }
                self.set_status(format!(
                    "Session {} of {}",
                    self.selected_session + 1,
                    self.filtered_indices.len()
                ));
                CommandResult::Executed
            }
            "global" | "G" => {
                self.global_mode = true;
                self.project_filter = None;
                self.apply_filter_and_sort();
                self.set_status("Global mode: all projects");
                CommandResult::Executed
            }
            "cwd" => {
                self.global_mode = false;
                self.project_filter = self.current_project.clone();
                self.apply_filter_and_sort();
                self.set_status("CWD mode: current project only");
                CommandResult::Executed
            }
            "clear" | "cls" => {
                self.search_query.clear();
                self.apply_filter_and_sort();
                self.set_status("Search cleared");
                CommandResult::Executed
            }
            other => CommandResult::Unknown(other.to_string()),
        }
    }
}

fn fuzzy_score(query: &str, candidate: &str) -> Option<isize> {
    let q = query.to_lowercase();
    let c = candidate.to_lowercase();

    if q.is_empty() {
        return Some(0);
    }

    if c == q {
        return Some(1000);
    }
    if c.starts_with(&q) {
        return Some(500 + (c.len() as isize));
    }
    if c.contains(&q) {
        return Some(200 + (c.len() as isize));
    }

    let mut qi = 0;
    let qc: Vec<char> = q.chars().collect();

    for (_ci, ch) in c.char_indices() {
        if qi < qc.len() && ch == qc[qi] {
            qi += 1;
        }
    }

    if qi == qc.len() {
        Some(100 - (c.len() as isize))
    } else {
        None
    }
}

fn sort_label(sort: SortBy) -> &'static str {
    match sort {
        SortBy::Updated => "updated",
        SortBy::Created => "created",
        SortBy::Title => "title",
    }
}
