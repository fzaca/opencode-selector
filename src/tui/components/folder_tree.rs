use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::tui::app::App;
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border())
        .title(" Folders ")
        .title_style(theme.highlight());

    if app.folders.is_empty() {
        let empty = List::new(vec![ListItem::new("No folders")]).block(block);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .folders
        .iter()
        .enumerate()
        .map(|(idx, folder)| {
            let style = if idx == app.selected_folder {
                theme.selected()
            } else {
                theme.default_style()
            };
            let count = app
                .session_folder_map
                .values()
                .filter(|&fid| fid == &folder.id)
                .count();
            let label = if folder.id == "inbox" {
                format!("📥 {} ({})", folder.name, count)
            } else if folder.id == "archive" {
                format!("🗃  {} ({})", folder.name, count)
            } else {
                format!("📁 {} ({})", folder.name, count)
            };
            ListItem::new(Line::from(vec![Span::styled(label, style)])).style(style)
        })
        .collect();

    let mut state = ListState::default().with_selected(Some(app.selected_folder));
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut state);
}
