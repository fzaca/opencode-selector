use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::tui::app::App;
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, theme: Theme) {
    let session = match app.current_session() {
        Some(s) => s.clone(),
        None => return,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(format!(" Preview: {} ", session.display_title()))
        .title_style(theme.highlight());

    let inner = block.inner(area);
    let available = inner.height.saturating_sub(1) as usize;

    let mut lines = vec![
        Line::from(vec![
            Span::styled("ID: ", theme.accent()),
            Span::raw(&session.id),
        ]),
        Line::from(vec![
            Span::styled("Slug: ", theme.accent()),
            Span::raw(&session.slug),
        ]),
        Line::from(vec![
            Span::styled("Project: ", theme.accent()),
            Span::raw(&session.project_name),
        ]),
        Line::from(vec![
            Span::styled("Agent: ", theme.accent()),
            Span::raw(session.agent.as_deref().unwrap_or("-")),
        ]),
        Line::from(vec![
            Span::styled("Model: ", theme.accent()),
            Span::raw(session.model_name.as_deref().unwrap_or("-")),
        ]),
        Line::from(vec![
            Span::styled("Files changed: ", theme.accent()),
            Span::raw(session.summary_files.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Updated: ", theme.accent()),
            Span::raw(
                session
                    .updated_at
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string(),
            ),
        ]),
        Line::raw(""),
        Line::styled("First message:", theme.accent()),
    ];

    if let Some(ref preview) = session.first_message_preview {
        for line in preview.lines() {
            lines.push(Line::raw(line.to_string()));
        }
    } else {
        lines.push(Line::styled("No preview available.", theme.dim()));
    }

    let total = lines.len();
    let max_scroll = total.saturating_sub(available);
    if app.preview_scroll as usize > max_scroll {
        app.preview_scroll = max_scroll as u16;
    }
    let scroll = app.preview_scroll as usize;

    let visible: Vec<Line> = lines.into_iter().skip(scroll).take(available).collect();

    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);
    f.render_widget(Paragraph::new(visible).wrap(Wrap { trim: false }), inner);

    if total > available {
        let pct = scroll as f64 / max_scroll.max(1) as f64;
        let pos = (pct * (available.saturating_sub(1) as f64)).round() as u16;
        let mut sb = String::with_capacity(inner.width as usize);
        for i in 0..inner.width.saturating_sub(2) {
            sb.push(if i == pos { '█' } else { '░' });
        }
        let status = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {}:{} ", scroll + 1, total), theme.dim()),
            Span::raw(sb),
        ]))
        .alignment(Alignment::Right);
        let status_area = Rect {
            x: inner.x,
            y: inner.y + inner.height - 1,
            width: inner.width,
            height: 1,
        };
        f.render_widget(status, status_area);
    }
}
