use ratatui::{
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::db::Session;
use crate::tui::app::{App, SortBy};
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .title(format!(
            " Sessions ({}) [{}] ",
            app.filtered_indices.len(),
            sort_label(app.sort_by)
        ))
        .title_style(theme.highlight());

    if app.filtered_indices.is_empty() {
        let empty = List::new(vec![ListItem::new("No sessions match")]).block(block);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .enumerate()
        .map(|(visible_idx, &session_idx)| {
            let session = &app.sessions[session_idx];
            ListItem::new(session_line(session, theme)).style(
                if visible_idx == app.selected_session {
                    theme.selected()
                } else {
                    theme.default_style()
                },
            )
        })
        .collect();

    let mut state = ListState::default().with_selected(Some(app.selected_session));
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected())
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut state);
}

fn session_line<'a>(session: &Session, theme: Theme) -> Line<'a> {
    let title = session.display_title();
    let meta = format!(
        "{} · {}",
        format_time(session.updated_at),
        session.project_name,
    );

    Line::from(vec![
        Span::styled(
            format!("{:<50} ", truncate(title, 48)),
            theme.accent().add_modifier(Modifier::BOLD),
        ),
        Span::styled(meta, theme.dim()),
    ])
}

fn sort_label(sort: SortBy) -> &'static str {
    match sort {
        SortBy::Updated => "updated",
        SortBy::Created => "created",
        SortBy::Title => "title",
    }
}

fn format_time(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}

fn truncate(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max_len).collect::<String>())
    }
}
