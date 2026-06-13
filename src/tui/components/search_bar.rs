use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::tui::app::App;
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &App, area: Rect, theme: Theme) {
    let prompt = if matches!(app.input_mode, crate::tui::app::InputMode::Search) {
        "/"
    } else {
        "search"
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(format!(" {} ", prompt))
        .title_style(theme.highlight());

    let text = Line::from(vec![
        Span::raw(app.search_query.clone()),
        Span::styled("█", theme.accent()),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
