use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(" Help ")
        .title_style(theme.highlight());

    let inner = block.inner(area);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    let left = build_group(
        "Navigation",
        &[
            ("↑ / ↓ or k / j", "Move selection"),
            ("PgUp / PgDn", "Scroll page"),
            ("gg / G", "First / last session"),
            ("h / l or ← / →", "Focus folders"),
        ],
        theme,
    );

    let right_top = build_group(
        "Actions",
        &[
            ("Enter", "Open session"),
            ("n", "New session"),
            ("p", "Preview"),
            ("r", "Rename"),
            ("m", "Move to folder"),
            ("d", "Archive"),
            ("D", "Delete"),
        ],
        theme,
    );

    let right_bottom = build_group(
        "Filters & Sort",
        &[
            ("P", "Toggle project filter"),
            ("F", "Toggle folders"),
            ("a", "Jump to All folder"),
            ("N", "New folder"),
            ("/", "Search"),
            ("s", "Cycle sort"),
        ],
        theme,
    );

    let right = [right_top, vec![Line::from("")], right_bottom].concat();

    f.render_widget(Paragraph::new(left), chunks[0]);
    f.render_widget(Paragraph::new(right), chunks[1]);
}

fn build_group<'a>(title: &'a str, bindings: &[(&'a str, &'a str)], theme: Theme) -> Vec<Line<'a>> {
    let mut lines = vec![Line::from(vec![Span::styled(
        format!(" {} ", title),
        theme.highlight(),
    )])];

    for (key, desc) in bindings {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<16}", key),
                theme.accent().add_modifier(Modifier::BOLD),
            ),
            Span::styled(*desc, theme.default_style()),
        ]));
    }

    lines
}
