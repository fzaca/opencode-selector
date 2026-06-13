use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

use crate::db::Session;
use crate::tui::app::{App, SortBy};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(format!(
            " Sessions ({}) {} ",
            app.filtered_indices.len(),
            sort_badge(app.sort_by)
        ))
        .title_style(theme.highlight());

    if app.filtered_indices.is_empty() {
        let empty = Paragraph::new("No sessions match")
            .alignment(Alignment::Center)
            .block(block);
        f.render_widget(empty, area);
        return;
    }

    let inner_width = area.width.saturating_sub(6).max(10) as usize;

    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .enumerate()
        .map(|(visible_idx, &session_idx)| {
            let session = &app.sessions[session_idx];
            let is_selected = visible_idx == app.selected_session;
            let prefix = if is_selected { "▶ " } else { "  " };
            ListItem::new(session_item(
                session,
                inner_width,
                theme,
                is_selected,
                prefix,
            ))
            .style(if is_selected {
                theme.selected()
            } else {
                theme.default_style()
            })
        })
        .collect();

    let mut state = ListState::default().with_selected(Some(app.selected_session));
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected())
        .highlight_symbol("");

    f.render_stateful_widget(list, area, &mut state);
}

fn session_item<'a>(
    session: &Session,
    width: usize,
    theme: Theme,
    selected: bool,
    prefix: &'a str,
) -> Vec<Line<'a>> {
    let title_style = if selected {
        theme.selected()
    } else {
        theme.accent().add_modifier(Modifier::BOLD)
    };
    let meta_style = if selected {
        theme.selected_dim()
    } else {
        theme.dim()
    };

    let title = truncate(session.display_title(), width);
    let meta = format!(
        "{}  {}",
        relative_time(session.updated_at),
        session.project_name,
    );

    let mut lines = vec![
        Line::from(vec![Span::raw(prefix), Span::styled(title, title_style)]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(truncate(&meta, width), meta_style),
        ]),
    ];

    if let Some(preview) = session.first_message_preview.as_deref() {
        let preview_text = truncate(preview, width);
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                preview_text,
                if selected {
                    theme.selected_dim()
                } else {
                    theme.muted()
                },
            ),
        ]));
    }

    lines
}

fn sort_badge(sort: SortBy) -> String {
    match sort {
        SortBy::Updated => "↻ updated".to_string(),
        SortBy::Created => "+ created".to_string(),
        SortBy::Title => "≡ title".to_string(),
    }
}

fn relative_time(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(dt);

    if diff.num_seconds() < 60 {
        "just now".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{}m ago", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_days() < 7 {
        format!("{}d ago", diff.num_days())
    } else {
        dt.format("%Y-%m-%d").to_string()
    }
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
