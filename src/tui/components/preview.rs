use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
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
    let available = inner.height as usize;

    let mut lines = vec![
        meta_line("ID", &session.id, theme),
        meta_line("Slug", &session.slug, theme),
        meta_line("Project", &session.project_name, theme),
        meta_line("Agent", session.agent.as_deref().unwrap_or("-"), theme),
        meta_line("Model", session.model_name.as_deref().unwrap_or("-"), theme),
        meta_line(
            "Files changed",
            &session
                .summary_files
                .map(|f| f.to_string())
                .unwrap_or_else(|| "-".to_string()),
            theme,
        ),
        meta_line(
            "Updated",
            &session
                .updated_at
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
            theme,
        ),
    ];

    lines.push(Line::raw(""));

    if session.messages.is_empty() {
        if let Some(ref preview) = session.first_message_preview {
            lines.push(Line::styled("Messages:", theme.accent()));
            for line in preview.lines() {
                lines.push(Line::raw(line.to_string()));
            }
        } else {
            lines.push(Line::styled("(loading messages...)", theme.dim()));
        }
    } else {
        for (role, text) in &session.messages {
            let role_style = if role == "assistant" {
                theme.success().dim()
            } else {
                theme.accent().dim()
            };
            lines.push(Line::styled(format!("── {role} ──"), role_style));
            for line in text.lines() {
                lines.push(Line::raw(line.to_string()));
            }
            lines.push(Line::raw(""));
        }
    }

    let total = lines.len();
    let max_scroll = total.saturating_sub(available);
    if app.preview_scroll as usize > max_scroll {
        app.preview_scroll = max_scroll as u16;
    }
    let scroll = app.preview_scroll as usize;

    let visible: Vec<Line> = lines.into_iter().skip(scroll).take(available).collect();

    f.render_widget(block.clone(), area);
    f.render_widget(
        Paragraph::new(visible.clone())
            .style(theme.default_style())
            .wrap(Wrap { trim: false }),
        inner,
    );

    if total > available {
        f.render_widget(
            Paragraph::new(format!(" {}:{} ", scroll + 1, total))
                .style(theme.dim())
                .alignment(Alignment::Right),
            Rect {
                x: inner.x,
                y: inner.y + inner.height - 1,
                width: inner.width,
                height: 1,
            },
        );
    }
}

fn meta_line<'a>(label: &str, value: &str, theme: Theme) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{label}: "), theme.accent()),
        Span::raw(value.to_string()),
    ])
}
