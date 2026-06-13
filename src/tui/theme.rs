use ratatui::style::{Color, Modifier, Style};

/// Theme that adapts to the terminal's default 16-color palette.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub border: Color,
    pub highlight: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
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
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
        }
    }
}

impl Theme {
    pub fn default_style(self) -> Style {
        Style::default()
            .fg(self.foreground)
            .bg(self.background)
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

    pub fn dim(self) -> Style {
        Style::default().fg(self.accent_dim).bg(self.background)
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
}
