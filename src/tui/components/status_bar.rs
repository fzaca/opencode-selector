use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::tui::app::{App, InputMode, Screen, SortBy};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border());

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(ref msg) = app.status_message {
        let line = Line::from(vec![Span::styled(
            truncate(msg, inner.width as usize),
            theme.warning(),
        )]);
        let paragraph = Paragraph::new(line);
        f.render_widget(paragraph, inner);
        return;
    }

    let show_badge = should_show_mode_badge(app);

    if show_badge {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(mode_width(app)),
                Constraint::Min(12),
                Constraint::Length(shortcut_width(app)),
            ])
            .split(inner);

        let mode = Line::from(vec![Span::styled(
            format!(" {} ", mode_label(app)),
            theme.badge(),
        )]);
        f.render_widget(Paragraph::new(mode), chunks[0]);

        let context = Line::from(vec![
            Span::styled("filter: ", theme.dim()),
            Span::styled(context_label(app), theme.accent()),
            Span::styled("  sort: ", theme.dim()),
            Span::styled(sort_label(app.sort_by), theme.accent()),
        ]);
        f.render_widget(Paragraph::new(context), chunks[1]);

        let shortcuts = Line::from(shortcuts(app, theme));
        f.render_widget(
            Paragraph::new(shortcuts).alignment(Alignment::Right),
            chunks[2],
        );
    } else {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(12), Constraint::Length(shortcut_width(app))])
            .split(inner);

        let context = Line::from(vec![
            Span::styled("filter: ", theme.dim()),
            Span::styled(context_label(app), theme.accent()),
            Span::styled("  sort: ", theme.dim()),
            Span::styled(sort_label(app.sort_by), theme.accent()),
        ]);
        f.render_widget(Paragraph::new(context), chunks[0]);

        let shortcuts = Line::from(shortcuts(app, theme));
        f.render_widget(
            Paragraph::new(shortcuts).alignment(Alignment::Right),
            chunks[1],
        );
    }
}

fn should_show_mode_badge(app: &App) -> bool {
    !(app.screen == Screen::Main && app.input_mode == InputMode::Normal)
}

fn context_label(app: &App) -> String {
    if app.global_mode {
        "global".to_string()
    } else {
        "cwd".to_string()
    }
}

fn sort_label(sort: SortBy) -> &'static str {
    match sort {
        SortBy::Updated => "updated",
        SortBy::Created => "created",
        SortBy::Title => "title",
    }
}

fn mode_label(app: &App) -> String {
    match app.screen {
        Screen::Main => match app.input_mode {
            InputMode::Normal => "NORMAL".to_string(),
            InputMode::Search => "SEARCH".to_string(),
            InputMode::Rename => "RENAME".to_string(),
            InputMode::NewFolder => "NEW FOLDER".to_string(),
            InputMode::Confirm => "CONFIRM".to_string(),
            InputMode::Command => "COMMAND".to_string(),
        },
        Screen::Preview => "PREVIEW".to_string(),
        Screen::Help => "HELP".to_string(),
        Screen::ConfirmDelete => "DELETE".to_string(),
        Screen::Rename => "RENAME".to_string(),
        Screen::MoveToFolder => "MOVE".to_string(),
        Screen::NewFolder => "NEW FOLDER".to_string(),
    }
}

fn mode_width(app: &App) -> u16 {
    (mode_label(app).chars().count() + 2).max(8) as u16
}

fn shortcuts<'a>(app: &App, theme: Theme) -> Vec<Span<'a>> {
    let key = |k: &'a str| Span::styled(k, theme.accent().add_modifier(Modifier::BOLD));

    match app.input_mode {
        InputMode::Search => vec![
            key("Enter"),
            Span::raw(" search "),
            key("Esc"),
            Span::raw(" clear"),
        ],
        _ => match app.screen {
            Screen::Main => vec![key("?"), Span::raw(" help "), key("q"), Span::raw(" quit")],
            _ => vec![key("Esc"), Span::raw(" back")],
        },
    }
}

fn shortcut_width(app: &App) -> u16 {
    let spans = shortcuts(app, Theme::terminal());
    spans.iter().map(|s| s.width() as u16).sum::<u16>().max(14)
}

fn truncate(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        format!(
            "{}…",
            text.chars()
                .take(max_len.saturating_sub(1))
                .collect::<String>()
        )
    }
}
