use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
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
        f.render_widget(
            Paragraph::new("No sessions match")
                .alignment(Alignment::Center)
                .block(block),
            area,
        );
        return;
    }

    let inner = block.inner(area);
    let item_height = item_lines(app).saturating_add(1);
    let visible_rows = (inner.height as usize).saturating_sub(1);

    if visible_rows == 0 {
        return;
    }

    let total_items = app.filtered_indices.len();
    let max_scroll = total_items.saturating_sub(visible_rows / item_height);

    if app.selected_session > max_scroll {
        app.selected_session = max_scroll;
    }

    let scroll = app.selected_session;
    let inner_width = inner.width.saturating_sub(2).max(10) as usize;

    let mut y = inner.y;
    let mut visible_idx = scroll;
    let mut rendered = 0;

    while y + item_height as u16 <= inner.y + inner.height {
        if visible_idx >= total_items {
            break;
        }

        let session = &app.sessions[app.filtered_indices[visible_idx]];
        let is_sel = visible_idx == app.selected_session;

        let lines = session_item(session, inner_width, theme, is_sel);
        for (li, line) in lines.iter().enumerate() {
            let line_area = Rect {
                x: inner.x + 1,
                y: y + li as u16,
                width: inner.width.saturating_sub(2),
                height: 1,
            };
            f.render_widget(
                Paragraph::new(line.clone()).style(
                    if is_sel { theme.selected() } else { theme.default_style() },
                ),
                line_area,
            );
        }

        y += item_height as u16;
        visible_idx += 1;
        rendered += 1;
    }

    let scrollbar_area = Rect {
        x: inner.x + inner.width - 1,
        y: inner.y,
        width: 1,
        height: inner.height,
    };
    if total_items > visible_rows / item_height {
        let track_height = inner.height as usize;
        let thumb_pos = if total_items > 0 {
            scroll * track_height / total_items
        } else {
            0
        };
        let thumb_end = if total_items > 0 {
            ((scroll + rendered) * track_height).div_ceil(total_items)
        } else {
            0
        };

        for ty in 0..track_height {
            if ty >= thumb_pos && ty < thumb_end {
                f.render_widget(
                    Paragraph::new("█").style(theme.accent()),
                    Rect {
                        x: scrollbar_area.x,
                        y: scrollbar_area.y + ty as u16,
                        width: 1,
                        height: 1,
                    },
                );
            } else {
                f.render_widget(
                    Paragraph::new("░").style(theme.dim()),
                    Rect {
                        x: scrollbar_area.x,
                        y: scrollbar_area.y + ty as u16,
                        width: 1,
                        height: 1,
                    },
                );
            }
        }
    }
}

fn item_lines(app: &App) -> usize {
    app.filtered_indices
        .first()
        .and_then(|&idx| app.sessions.get(idx))
        .map(item_line_count)
        .unwrap_or(2)
}

fn item_line_count(session: &Session) -> usize {
    if session.first_message_preview.is_some() { 3 } else { 2 }
}

fn session_item<'a>(
    session: &Session,
    width: usize,
    theme: Theme,
    selected: bool,
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

    let prefix = Span::styled("  ", theme.dim());
    let title = truncate(session.display_title(), width.saturating_sub(2));
    let meta = format!(
        "{} · {}",
        short_ago(session.updated_at),
        session.project_name,
    );

    let mut lines = vec![
        Line::from(vec![prefix.clone(), Span::styled(title, title_style)]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(truncate(&meta, width.saturating_sub(2)), meta_style),
        ]),
    ];

    if let Some(preview) = session.first_message_preview.as_deref() {
        let preview_text = truncate(preview, width.saturating_sub(2));
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
        SortBy::Updated => "↻ updated",
        SortBy::Created => "+ created",
        SortBy::Title => "≡ title",
    }
    .to_string()
}

fn short_ago(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(dt);

    if diff.num_seconds() < 60 {
        "now".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{}m", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h", diff.num_hours())
    } else if diff.num_days() < 7 {
        format!("{}d", diff.num_days())
    } else {
        dt.format("%m/%d").to_string()
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
