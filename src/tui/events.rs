use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind};

use crate::db::SessionRepository;
use crate::folders::FolderStore;
use crate::tui::app::{App, InputMode, Screen};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Continue,
    LaunchSession { id: String },
    LaunchNew,
    Quit,
}

pub fn next_event(
    app: &mut App,
    repo: &SessionRepository,
    store: &mut FolderStore,
) -> io::Result<AppEvent> {
    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        return handle_key(app, repo, store, key);
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollDown => {
                        app.move_selection_down();
                        return Ok(AppEvent::Continue);
                    }
                    MouseEventKind::ScrollUp => {
                        app.move_selection_up();
                        return Ok(AppEvent::Continue);
                    }
                    _ => {}
                },
                Event::Resize(_, _) => return Ok(AppEvent::Continue),
                _ => {}
            }
        }
    }
}

fn handle_key(
    app: &mut App,
    repo: &SessionRepository,
    store: &mut FolderStore,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    match app.input_mode {
        InputMode::Search => return handle_search_key(app, key),
        InputMode::Rename => return handle_rename_key(app, repo, key),
        InputMode::NewFolder => return handle_new_folder_key(app, store, key),
        InputMode::Confirm => return handle_confirm_key(app, repo, store, key),
        InputMode::Normal => {}
    }

    match app.screen {
        Screen::Main => handle_main_key(app, repo, store, key),
        Screen::Preview => handle_preview_key(app, key),
        Screen::Help => {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                app.screen = Screen::Main;
            }
            Ok(AppEvent::Continue)
        }
        Screen::MoveToFolder => handle_move_folder_key(app, store, key),
        _ => Ok(AppEvent::Continue),
    }
}

fn handle_main_key(
    app: &mut App,
    _repo: &SessionRepository,
    store: &mut FolderStore,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    if app.pending_key == Some('g') && key.code == KeyCode::Char('g') {
        app.selected_session = 0;
        app.pending_key = None;
        return Ok(AppEvent::Continue);
    }
    app.pending_key = None;

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(AppEvent::Quit),
        KeyCode::Char('?') => app.screen = Screen::Help,
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Search;
            app.search_query.clear();
        }
        KeyCode::Char('n') => return Ok(AppEvent::LaunchNew),
        KeyCode::Char('p') => {
            if app.current_session().is_some() {
                app.screen = Screen::Preview;
            }
        }
        KeyCode::Char('r') => start_rename(app),
        KeyCode::Char('m') => start_move(app),
        KeyCode::Char('d') => archive_current(app, store),
        KeyCode::Char('D') => start_delete(app),
        KeyCode::Char('s') => app.cycle_sort(),
        KeyCode::Char('N') => start_new_folder(app),
        KeyCode::Char('g') => app.pending_key = Some('g'),
        KeyCode::Char('G') => {
            app.selected_session = app.filtered_indices.len().saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
        KeyCode::Left | KeyCode::Char('h') => app.move_folder_up(),
        KeyCode::Right | KeyCode::Char('l') => app.move_folder_down(),
        KeyCode::PageDown => {
            for _ in 0..10 {
                app.move_selection_down();
            }
        }
        KeyCode::PageUp => {
            for _ in 0..10 {
                app.move_selection_up();
            }
        }
        KeyCode::Home => app.selected_session = 0,
        KeyCode::End => app.selected_session = app.filtered_indices.len().saturating_sub(1),
        KeyCode::Enter => {
            if let Some(session) = app.current_session() {
                return Ok(AppEvent::LaunchSession {
                    id: session.id.clone(),
                });
            }
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn handle_search_key(app: &mut App, key: KeyEvent) -> io::Result<AppEvent> {
    match key.code {
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.search_query.clear();
            app.apply_filter_and_sort();
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.apply_filter_and_sort();
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.apply_filter_and_sort();
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn handle_preview_key(app: &mut App, key: KeyEvent) -> io::Result<AppEvent> {
    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') || key.code == KeyCode::Char('p') {
        app.screen = Screen::Main;
    }
    Ok(AppEvent::Continue)
}

fn start_rename(app: &mut App) {
    if let Some(session) = app.current_session().cloned() {
        app.rename_buffer = session.title.clone();
        app.confirm_message = format!("Rename '{}' to:", session.display_title());
        app.screen = Screen::Rename;
        app.input_mode = InputMode::Rename;
    }
}

fn handle_rename_key(
    app: &mut App,
    repo: &SessionRepository,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    match key.code {
        KeyCode::Enter => {
            if let Some(session) = app.current_session().cloned() {
                let new_title = app.rename_buffer.trim().to_string();
                if !new_title.is_empty() && new_title != session.title {
                    if let Err(e) = repo.rename_session(&session.id, &new_title) {
                        app.set_status(format!("Rename failed: {e}"));
                    } else {
                        if let Some(s) = app
                            .sessions
                            .iter_mut()
                            .find(|s| s.id == session.id)
                        {
                            s.title = new_title;
                        }
                        app.apply_filter_and_sort();
                        app.set_status("Session renamed");
                    }
                }
            }
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.rename_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.rename_buffer.push(c);
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn start_delete(app: &mut App) {
    if let Some(session) = app.current_session().cloned() {
        app.pending_delete_id = Some(session.id.clone());
        app.confirm_message = format!(
            "Permanently delete '{}' ?",
            session.display_title()
        );
        app.screen = Screen::ConfirmDelete;
        app.input_mode = InputMode::Confirm;
    }
}

fn handle_confirm_key(
    app: &mut App,
    repo: &SessionRepository,
    store: &mut FolderStore,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if app.screen == Screen::ConfirmDelete {
                if let Some(id) = app.pending_delete_id.take() {
                    if let Err(e) = repo.delete_session(&id) {
                        app.set_status(format!("Delete failed: {e}"));
                    } else {
                        app.sessions.retain(|s| s.id != id);
                        app.session_folder_map.remove(&id);
                        let _ = store.unassign_session(&id);
                        app.set_status("Session deleted");
                        app.apply_filter_and_sort();
                    }
                }
            }
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
            app.pending_delete_id = None;
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn archive_current(app: &mut App, store: &mut FolderStore) {
    if let Some(session) = app.current_session().cloned() {
        if let Err(e) = store.move_session(&session.id, "archive") {
            app.set_status(format!("Archive failed: {e}"));
        } else {
            app.session_folder_map
                .insert(session.id.clone(), "archive".to_string());
            app.set_status("Session archived");
            app.apply_filter_and_sort();
        }
    }
}

fn start_move(app: &mut App) {
    if app.current_session().is_some() {
        app.screen = Screen::MoveToFolder;
        app.confirm_message = "Select folder:".to_string();
    }
}

fn handle_move_folder_key(
    app: &mut App,
    store: &mut FolderStore,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.screen = Screen::Main;
        }
        KeyCode::Up | KeyCode::Char('k') => app.move_folder_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_folder_down(),
        KeyCode::Enter => {
            if let (Some(session), Some(folder)) =
                (app.current_session().cloned(), app.current_folder().cloned())
            {
                if let Err(e) = store.move_session(&session.id, &folder.id) {
                    app.set_status(format!("Move failed: {e}"));
                } else {
                    app.session_folder_map
                        .insert(session.id.clone(), folder.id.clone());
                    app.set_status(format!("Moved to {}", folder.name));
                    app.apply_filter_and_sort();
                }
            }
            app.screen = Screen::Main;
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn start_new_folder(app: &mut App) {
    app.new_folder_buffer.clear();
    app.screen = Screen::NewFolder;
    app.input_mode = InputMode::NewFolder;
}

fn handle_new_folder_key(
    app: &mut App,
    store: &mut FolderStore,
    key: KeyEvent,
) -> io::Result<AppEvent> {
    match key.code {
        KeyCode::Enter => {
            let name = app.new_folder_buffer.trim().to_string();
            if !name.is_empty() {
                let id = slugify(&name);
                if let Err(e) = store.add_folder(crate::folders::Folder::new(&id, &name)) {
                    app.set_status(format!("Create folder failed: {e}"));
                } else {
                    app.folders.push(crate::folders::Folder::new(&id, &name));
                    app.set_status(format!("Created folder {}", name));
                }
            }
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.screen = Screen::Main;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.new_folder_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.new_folder_buffer.push(c);
        }
        _ => {}
    }
    Ok(AppEvent::Continue)
}

fn slugify(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_produces_simple_ids() {
        assert_eq!(slugify("Work Stuff"), "work-stuff");
        assert_eq!(slugify("Personal!!!"), "personal");
    }
}
