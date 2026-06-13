use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .title(" Help ")
        .title_style(theme.highlight());

    let lines = vec![
        Line::from(vec![Span::styled("Navigation", theme.accent())]),
        Line::from("↑ / ↓ or k / j   Move selection"),
        Line::from("PgUp / PgDn      Scroll page"),
        Line::from("gg / G           Go to first / last"),
        Line::from(""),
        Line::from(vec![Span::styled("Actions", theme.accent())]),
        Line::from("Enter            Open selected session in opencode"),
        Line::from("n                Create a new opencode session"),
        Line::from("p                Open full-screen preview"),
        Line::from("r                Rename session"),
        Line::from("m                Move session to folder"),
        Line::from("d                Move session to Archive"),
        Line::from("D                Permanently delete session"),
        Line::from("a                Jump to All folder"),
        Line::from(""),
        Line::from(vec![Span::styled("Search & Sort", theme.accent())]),
        Line::from("/                Start search"),
        Line::from("s                Cycle sort (updated / created / title)"),
        Line::from(""),
        Line::from(vec![Span::styled("Folders", theme.accent())]),
        Line::from("h / l or ← / →   Switch focus to folders"),
        Line::from("N                Create new folder"),
        Line::from(""),
        Line::from(vec![Span::styled("General", theme.accent())]),
        Line::from("?                Toggle this help"),
        Line::from("Esc / q          Back or quit"),
        Line::from("Mouse            Click to select and scroll"),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
