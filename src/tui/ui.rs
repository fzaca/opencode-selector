use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::tui::app::{App, InputMode, Screen};
use crate::tui::components::{
    folder_tree, help, preview, search_bar, session_list, status_bar,
};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: Theme) {
    let size = f.area();

    match app.screen {
        Screen::Help => {
            let area = centered_rect(70, 80, size);
            help::draw(f, area, theme);
        }
        Screen::Preview => {
            if let Some(session) = app.current_session().cloned() {
                preview::draw(f, &session, size, theme);
            } else {
                app.screen = Screen::Main;
                draw_main(f, app, theme);
            }
        }
        Screen::ConfirmDelete | Screen::Rename | Screen::MoveToFolder | Screen::NewFolder => {
            draw_main(f, app, theme);
            let area = centered_rect(60, 20, size);
            draw_modal(f, app, area, theme);
        }
        Screen::Main => {
            draw_main(f, app, theme);
        }
    }
}

fn draw_main(f: &mut Frame, app: &mut App, theme: Theme) {
    let size = f.area();
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(size);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(main_chunks[0]);

    folder_tree::draw(f, app, body_chunks[0], theme);
    session_list::draw(f, app, body_chunks[1], theme);

    if matches!(app.input_mode, InputMode::Search) && app.screen == Screen::Main {
        let search_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(main_chunks[1])[0];
        search_bar::draw(f, app, search_area, theme);
    } else {
        status_bar::draw(f, app, main_chunks[1], theme);
    }
}

fn draw_modal(f: &mut Frame, app: &App, area: Rect, theme: Theme) {
    use ratatui::widgets::{Block, Borders, Clear, Paragraph};

    let (title, content) = match app.screen {
        Screen::ConfirmDelete => (
            " Confirm Delete ",
            format!("{}\n\nPress y to confirm or Esc to cancel.", app.confirm_message),
        ),
        Screen::Rename => (
            " Rename Session ",
            format!("{}\n\n{}", app.confirm_message, app.rename_buffer),
        ),
        Screen::NewFolder => (
            " New Folder ",
            format!("Enter folder name:\n\n{}", app.new_folder_buffer),
        ),
        Screen::MoveToFolder => (
            " Move to Folder ",
            format!("{}\n\nUse ↑/↓ and Enter to select.", app.confirm_message),
        ),
        _ => return,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .title(title)
        .title_style(theme.highlight());

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
