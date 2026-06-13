use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::db::Session;
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, session: &Session, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(format!(" Preview: {} ", session.display_title()))
        .title_style(theme.highlight());

    let mut lines = vec![
        Line::from(vec![
            Span::styled("ID: ", theme.accent()),
            Span::raw(session.id.clone()),
        ]),
        Line::from(vec![
            Span::styled("Slug: ", theme.accent()),
            Span::raw(session.slug.clone()),
        ]),
        Line::from(vec![
            Span::styled("Project: ", theme.accent()),
            Span::raw(session.project_name.clone()),
        ]),
        Line::from(vec![
            Span::styled("Agent: ", theme.accent()),
            Span::raw(session.agent.clone().unwrap_or_else(|| "-".to_string())),
        ]),
        Line::from(vec![
            Span::styled("Model: ", theme.accent()),
            Span::raw(
                session
                    .model_name
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
            ),
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
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled("First message:", theme.accent())]),
    ];

    if let Some(ref preview) = session.first_message_preview {
        for line in preview.lines() {
            lines.push(Line::raw(line.to_string()));
        }
    } else {
        lines.push(Line::styled("No preview available.", theme.dim()));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}
