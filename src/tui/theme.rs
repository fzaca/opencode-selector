use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::BorderType;

/// Theme that adapts to the terminal's default 16-color palette.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub border: Color,
    pub highlight: Color,
    pub highlight_dim: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub muted: Color,
}

impl Theme {
    /// Use the terminal's default foreground/background colors and standard
    /// ANSI colors for accents. This makes the app feel native in any terminal
    /// theme.
    pub fn terminal() -> Self {
        Self {
            background: Color::Reset,
            foreground: Color::Reset,
            accent: Color::Cyan,
            accent_dim: Color::DarkGray,
            border: Color::DarkGray,
            highlight: Color::Blue,
            highlight_dim: Color::DarkGray,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            muted: Color::DarkGray,
        }
    }

    pub fn border_type(self) -> BorderType {
        BorderType::Rounded
    }
}

impl Theme {
    pub fn default_style(self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }

    pub fn accent(self) -> Style {
        Style::default().fg(self.accent).bg(self.background)
    }

    pub fn highlight(self) -> Style {
        Style::default()
            .fg(self.highlight)
            .bg(self.background)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected(self) -> Style {
        Style::default()
            .bg(self.highlight)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_dim(self) -> Style {
        Style::default().bg(self.highlight).fg(Color::White)
    }

    pub fn dim(self) -> Style {
        Style::default().fg(self.accent_dim).bg(self.background)
    }

    pub fn muted(self) -> Style {
        Style::default().fg(self.muted).bg(self.background)
    }

    pub fn error(self) -> Style {
        Style::default().fg(self.error).bg(self.background)
    }

    pub fn warning(self) -> Style {
        Style::default().fg(self.warning).bg(self.background)
    }

    pub fn success(self) -> Style {
        Style::default().fg(self.success).bg(self.background)
    }

    pub fn border(self) -> Style {
        Style::default().fg(self.border).bg(self.background)
    }

    pub fn badge(self) -> Style {
        Style::default()
            .fg(Color::White)
            .bg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }
}
