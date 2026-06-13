use ratatui::{
    Frame,
    layout::Rect,
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
            Paragraph::new("No sessions match").block(block),
            area,
        );
        return;
    }

    let inner = block.inner(area);
    let inner_width = inner.width.saturating_sub(2).max(10) as usize;
    let item_height = 3;
    let visible_rows = (inner.height as usize) / item_height;

    if visible_rows == 0 {
        f.render_widget(block, area);
        return;
    }

    let total = app.filtered_indices.len();
    let max_scroll = total.saturating_sub(visible_rows);
    if app.selected_session > max_scroll {
        app.selected_session = max_scroll;
    }

    let scroll = app.selected_session;
    let mut y = inner.y;

    for i in scroll..total.min(scroll + visible_rows) {
        let session = &app.sessions[app.filtered_indices[i]];
        let is_sel = i == app.selected_session;

        for (li, line) in session_item(session, inner_width, theme, is_sel)
            .into_iter()
            .enumerate()
        {
            let cell = Rect {
                x: inner.x + 1,
                y: y + li as u16,
                width: inner.width.saturating_sub(2),
                height: 1,
            };
            f.render_widget(
                Paragraph::new(line).style(if is_sel {
                    theme.selected()
                } else {
                    theme.default_style()
                }),
                cell,
            );
        }

        y += item_height as u16;
    }

    if total > visible_rows {
        let track_h = inner.height as usize;
        let thumb = scroll * track_h / total;
        let thumb_end = ((scroll + visible_rows) * track_h).div_ceil(total).min(track_h);

        let x = inner.x + inner.width - 1;
        for ty in 0..track_h as u16 {
            let in_thumb = (ty as usize) >= thumb && (ty as usize) < thumb_end;
            f.render_widget(
                Paragraph::new(if in_thumb { "█" } else { "░" }).style(
                    if in_thumb { theme.accent() } else { theme.dim() },
                ),
                Rect { x, y: inner.y + ty, width: 1, height: 1 },
            );
        }
    }
}

fn session_item<'a>(
    session: &Session,
    width: usize,
    theme: Theme,
    selected: bool,
) -> Vec<Line<'a>> {
    let title_style = if selected {
        theme.selected().add_modifier(Modifier::BOLD)
    } else {
        theme.accent().add_modifier(Modifier::BOLD)
    };
    let meta_style = if selected {
        theme.selected_dim()
    } else {
        theme.dim()
    };

    let prefix = if selected {
        Span::styled("▸ ", theme.selected())
    } else {
        Span::styled("  ", theme.dim())
    };

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
