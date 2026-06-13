use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    widgets::{Block, Clear},
};

use crate::tui::app::{App, InputMode, Screen};
use crate::tui::components::{folder_tree, help, preview, search_bar, session_list, status_bar};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: Theme) {
    let size = f.area();

    f.render_widget(
        ratatui::widgets::Block::default().style(theme.default_style()),
        size,
    );

    match app.screen {
        Screen::Help => {
            let area = centered_rect(80, 85, size).inner(Margin::new(1, 1));
            help::draw(f, area, theme);
        }
        Screen::Preview => {
            if app.current_session().is_some() {
                f.render_widget(Clear, size);
                f.render_widget(Block::default().style(theme.default_style()), size);
                let area = centered_rect(90, 90, size);
                preview::draw(f, app, area, theme);
            } else {
                app.screen = Screen::Main;
                draw_main(f, app, theme);
            }
        }
        Screen::ConfirmDelete | Screen::Rename | Screen::MoveToFolder | Screen::NewFolder => {
            draw_main(f, app, theme);
            let area = centered_rect(60, 22, size);
            draw_modal(f, app, area, theme);
        }
        Screen::Main => {
            draw_main(f, app, theme);
        }
    }
}

fn draw_main(f: &mut Frame, app: &mut App, theme: Theme) {
    let size = f.area().inner(Margin::new(1, 1));

    let suggest_count = if app.input_mode == InputMode::Command {
        app.command_suggestions.len().min(6) as u16 + 2
    } else {
        0
    };
    let bottom_bar = if app.input_mode == InputMode::Command {
        suggest_count + 3
    } else {
        3
    };

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(bottom_bar)])
        .split(size);

    if app.folders_enabled {
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(22), Constraint::Percentage(78)])
            .split(main_chunks[0]);

        folder_tree::draw(f, app, body_chunks[0], theme);
        session_list::draw(f, app, body_chunks[1], theme);
    } else {
        session_list::draw(f, app, main_chunks[0], theme);
    }

    if app.input_mode == InputMode::Command && app.screen == Screen::Main {
        draw_command_bar(f, app, main_chunks[1], theme);
    } else if matches!(app.input_mode, InputMode::Search) && app.screen == Screen::Main {
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
            format!(
                "{}\n\nPress y to confirm or Esc to cancel.",
                app.confirm_message
            ),
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
        .border_type(theme.border_type())
        .border_style(theme.border())
        .title(title)
        .title_style(theme.highlight());

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn draw_command_bar(f: &mut Frame, app: &App, area: Rect, theme: Theme) {
    use ratatui::text::{Line, Span};
    use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

    f.render_widget(Clear, area);
    f.render_widget(Block::default().style(theme.default_style()), area);

    let has_suggestions = !app.command_suggestions.is_empty();
    let suggestion_rows = if has_suggestions {
        app.command_suggestions.len().min(6) as u16 + 2
    } else {
        0
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if has_suggestions {
            vec![
                Constraint::Length(suggestion_rows),
                Constraint::Length(3),
            ]
        } else {
            vec![Constraint::Length(3)]
        })
        .split(area);

    if !app.command_suggestions.is_empty() {
        let suggestion_area = chunks[0];
        let sblock = Block::default()
            .borders(Borders::ALL)
            .border_type(theme.border_type())
            .border_style(theme.border())
            .title(" Suggestions ")
            .title_style(theme.highlight());
        let sinner = sblock.inner(suggestion_area);
        f.render_widget(sblock, suggestion_area);

        let items: Vec<ListItem> = app
            .command_suggestions
            .iter()
            .enumerate()
            .take(6)
            .map(|(i, s)| {
                let style = if i == app.selected_suggestion {
                    theme.highlight()
                } else {
                    theme.default_style()
                };
                let desc_style = if i == app.selected_suggestion {
                    theme.badge()
                } else {
                    theme.dim()
                };
                let max_text = sinner.width.saturating_sub(2) as usize;
                let desc_width = 30usize.min(max_text.saturating_sub(18));
                let desc = if s.description.is_empty() || desc_width < 4 {
                    String::new()
                } else {
                    let truncated: String = s.description.chars().take(desc_width).collect();
                    truncated
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {:<16}", &s.text[..s.text.len().min(16)]), style),
                    Span::styled(desc, desc_style),
                ]))
            })
            .collect();

        let mut list_state = ListState::default().with_selected(Some(app.selected_suggestion));
        let list = List::new(items).highlight_style(theme.highlight());
        f.render_stateful_widget(list, sinner, &mut list_state);
    }

    let input_area = chunks[if has_suggestions { 1 } else { 0 }];
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme.border_type())
        .border_style(theme.border())
        .title(" Command ");
    let inner = input_block.inner(input_area);
    f.render_widget(input_block, input_area);

    let prompt = Span::styled(":", theme.accent());
    let cursor = Span::styled("\u{2588}", theme.highlight());
    let text = Line::from(vec![prompt, Span::raw(&app.command_buffer), cursor]);
    f.render_widget(Paragraph::new(text), inner);
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
