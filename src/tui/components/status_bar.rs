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

    let mode_badge = mode_label(app);
    let pos_label = position_label(app);

    let left_width = (mode_badge.chars().count() as u16 + 2).max(8);

    let right_width = shortcut_width(app);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left_width),
            Constraint::Min(12),
            Constraint::Length(right_width),
        ])
        .split(inner);

    let mode = Line::from(vec![Span::styled(
        format!(" {} ", mode_badge),
        theme.badge(),
    )]);
    f.render_widget(Paragraph::new(mode), chunks[0]);

    let mut center_spans = vec![
        Span::styled("pos: ", theme.dim()),
        Span::styled(pos_label, theme.accent()),
        Span::styled("  sort: ", theme.dim()),
        Span::styled(sort_label(app.sort_by), theme.accent()),
    ];
    if app.search_query.is_empty() && app.project_filter.is_none() {
        let total = app.filtered_indices.len();
        center_spans.push(Span::styled("  total: ", theme.dim()));
        center_spans.push(Span::styled(total.to_string(), theme.accent()));
    }
    if !app.search_query.is_empty() {
        let q = truncate(&app.search_query, 20);
        center_spans.push(Span::styled("  search: ", theme.dim()));
        center_spans.push(Span::styled(q, theme.success()));
    }
    if let Some(ref pid) = app.project_filter {
        let p = truncate(pid, 15);
        center_spans.push(Span::styled("  project: ", theme.dim()));
        center_spans.push(Span::styled(p, theme.success()));
    }

    let center = Line::from(center_spans);
    f.render_widget(Paragraph::new(center), chunks[1]);

    let shortcuts = Line::from(shortcuts(app, theme));
    f.render_widget(
        Paragraph::new(shortcuts).alignment(Alignment::Right),
        chunks[2],
    );
}

fn position_label(app: &App) -> String {
    if app.filtered_indices.is_empty() {
        "0/0".to_string()
    } else {
        format!(
            "{}/{}",
            app.selected_session + 1,
            app.filtered_indices.len()
        )
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
            Screen::Main => vec![
                key("?"),
                Span::raw(" help "),
                key("p"),
                Span::raw(" prev "),
                key("q"),
                Span::raw(" quit"),
            ],
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
