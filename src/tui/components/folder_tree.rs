use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

use crate::tui::app::App;
use crate::tui::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, theme: Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border())
        .title(" Folders ")
        .title_style(theme.highlight());

    if app.folders.is_empty() {
        let empty = Paragraph::new("No folders")
            .alignment(Alignment::Center)
            .block(block);
        f.render_widget(empty, area);
        return;
    }

    let inner_width = area.width.saturating_sub(4).max(6) as usize;

    let items: Vec<ListItem> = app
        .folders
        .iter()
        .enumerate()
        .map(|(idx, folder)| {
            let is_selected = idx == app.selected_folder;
            let style = if is_selected {
                theme.selected()
            } else {
                theme.default_style()
            };
            let count = if folder.id == "all" {
                app.sessions.len()
            } else {
                app.session_folder_map
                    .values()
                    .filter(|&fid| fid == &folder.id)
                    .count()
            };
            let icon = folder_icon(&folder.id);
            let label = format!(
                "{} {} ({})",
                icon,
                truncate(&folder.name, inner_width.saturating_sub(6)),
                count
            );
            ListItem::new(Line::from(vec![Span::styled(label, style)])).style(style)
        })
        .collect();

    let mut state = ListState::default().with_selected(Some(app.selected_folder));
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected())
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, area, &mut state);
}

fn folder_icon(id: &str) -> &'static str {
    match id {
        "all" => "▤",
        "archive" => "▩",
        _ => "▧",
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
