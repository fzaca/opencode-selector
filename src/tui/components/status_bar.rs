use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::{App, InputMode, Screen};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border());

    let mut spans = Vec::new();

    if let Some(ref msg) = app.status_message {
        spans.push(Span::styled(msg.clone(), theme.warning()));
    } else {
        spans.push(Span::styled(mode_label(app), theme.highlight()));
        spans.push(Span::raw(" | "));
        spans.extend(shortcuts(app, theme));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).block(block);
    f.render_widget(paragraph, area);
}

fn mode_label(app: &App) -> String {
    match app.screen {
        Screen::Main => match app.input_mode {
            InputMode::Normal => "NORMAL".to_string(),
            InputMode::Search => "SEARCH".to_string(),
            InputMode::Rename => "RENAME".to_string(),
            InputMode::NewFolder => "NEW FOLDER".to_string(),
            InputMode::Confirm => "CONFIRM".to_string(),
        },
        Screen::Preview => "PREVIEW".to_string(),
        Screen::Help => "HELP".to_string(),
        Screen::ConfirmDelete => "CONFIRM DELETE".to_string(),
        Screen::Rename => "RENAME".to_string(),
        Screen::MoveToFolder => "MOVE".to_string(),
        Screen::NewFolder => "NEW FOLDER".to_string(),
    }
}

fn shortcuts<'a>(app: &App, theme: Theme) -> Vec<Span<'a>> {
    match app.input_mode {
        InputMode::Search => vec![
            Span::raw("Enter: search  "),
            Span::styled("Esc", theme.accent().add_modifier(Modifier::BOLD)),
            Span::raw(": clear"),
        ],
        _ => match app.screen {
            Screen::Main => vec![
                Span::raw("↑↓/jk: navigate  "),
                Span::raw("Enter: open  "),
                Span::raw("/: search  "),
                Span::raw("p: preview  "),
                Span::raw("n: new  "),
                Span::raw("r: rename  "),
                Span::raw("m: move  "),
                Span::raw("d: archive  "),
                Span::raw("D: delete  "),
                Span::raw("?: help  "),
                Span::styled("q", theme.accent().add_modifier(Modifier::BOLD)),
                Span::raw(": quit"),
            ],
            _ => vec![
                Span::styled("Esc/q", theme.accent().add_modifier(Modifier::BOLD)),
                Span::raw(": back"),
            ],
        },
    }
}
