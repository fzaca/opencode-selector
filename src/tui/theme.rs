use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::BorderType;

use crate::config::ThemeConfig;
use crate::opencode_theme::OpencodeTheme;

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

    /// Build a theme from the user's config file, falling back to the terminal
    /// defaults for any color that is missing or invalid.
    pub fn from_config(config: Option<&ThemeConfig>) -> Self {
        let mut theme = Self::terminal();
        if let Some(config) = config {
            theme.apply_config(config);
        }
        theme
    }

    /// Build a theme from opencode's active theme, falling back to the terminal
    /// defaults for any color that is missing or invalid.
    pub fn from_opencode(opencode: &OpencodeTheme) -> Self {
        let mut theme = Self::terminal();
        theme.apply_opencode(opencode);
        theme
    }

    pub fn apply_config(&mut self, config: &ThemeConfig) {
        if let Some(c) = config.background.as_deref().and_then(parse_hex_color) {
            self.background = c;
        }
        if let Some(c) = config.foreground.as_deref().and_then(parse_hex_color) {
            self.foreground = c;
        }
        if let Some(c) = config.accent.as_deref().and_then(parse_hex_color) {
            self.accent = c;
        }
        if let Some(c) = config.accent_dim.as_deref().and_then(parse_hex_color) {
            self.accent_dim = c;
        }
        if let Some(c) = config.border.as_deref().and_then(parse_hex_color) {
            self.border = c;
        }
        if let Some(c) = config.highlight.as_deref().and_then(parse_hex_color) {
            self.highlight = c;
        }
        if let Some(c) = config.highlight_dim.as_deref().and_then(parse_hex_color) {
            self.highlight_dim = c;
        }
        if let Some(c) = config.error.as_deref().and_then(parse_hex_color) {
            self.error = c;
        }
        if let Some(c) = config.warning.as_deref().and_then(parse_hex_color) {
            self.warning = c;
        }
        if let Some(c) = config.success.as_deref().and_then(parse_hex_color) {
            self.success = c;
        }
        if let Some(c) = config.muted.as_deref().and_then(parse_hex_color) {
            self.muted = c;
        }
    }

    pub fn apply_opencode(&mut self, opencode: &OpencodeTheme) {
        if let Some(c) = opencode.background {
            self.background = c;
        }
        if let Some(c) = opencode.foreground {
            self.foreground = c;
        }
        if let Some(c) = opencode.accent {
            self.accent = c;
        }
        if let Some(c) = opencode.accent_dim {
            self.accent_dim = c;
        }
        if let Some(c) = opencode.border {
            self.border = c;
        }
        if let Some(c) = opencode.highlight {
            self.highlight = c;
        }
        if let Some(c) = opencode.highlight_dim {
            self.highlight_dim = c;
        }
        if let Some(c) = opencode.error {
            self.error = c;
        }
        if let Some(c) = opencode.warning {
            self.warning = c;
        }
        if let Some(c) = opencode.success {
            self.success = c;
        }
        if let Some(c) = opencode.muted {
            self.muted = c;
        }
    }

    pub fn border_type(self) -> BorderType {
        BorderType::Rounded
    }
}

/// Parse a hex color string as `#RRGGBB` or `#RGB` into a `ratatui::Color`.
pub(crate) fn parse_hex_color(value: &str) -> Option<Color> {
    let value = value.trim();
    if !value.starts_with('#') {
        return None;
    }
    let chars: Vec<char> = value.chars().skip(1).collect();
    let (r, g, b) = match chars.len() {
        3 => {
            let to_byte = |c: char| c.to_digit(16).map(|d| (d * 17) as u8);
            (to_byte(chars[0])?, to_byte(chars[1])?, to_byte(chars[2])?)
        }
        6 => {
            let parse =
                |i| u8::from_str_radix(&chars[i..i + 2].iter().collect::<String>(), 16).ok();
            (parse(0)?, parse(2)?, parse(4)?)
        }
        _ => return None,
    };
    Some(Color::Rgb(r, g, b))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_color_accepts_six_digit() {
        assert_eq!(parse_hex_color("#ff5733"), Some(Color::Rgb(255, 87, 51)));
        assert_eq!(parse_hex_color("#00FF00"), Some(Color::Rgb(0, 255, 0)));
    }

    #[test]
    fn parse_hex_color_accepts_three_digit() {
        assert_eq!(parse_hex_color("#f0f"), Some(Color::Rgb(255, 0, 255)));
        assert_eq!(parse_hex_color("#abc"), Some(Color::Rgb(170, 187, 204)));
    }

    #[test]
    fn parse_hex_color_rejects_invalid() {
        assert_eq!(parse_hex_color("ff5733"), None);
        assert_eq!(parse_hex_color("#gggggg"), None);
        assert_eq!(parse_hex_color("#12345"), None);
    }

    #[test]
    fn theme_from_config_overrides_defaults() {
        let config = ThemeConfig {
            accent: Some("#ff5733".to_string()),
            background: Some("#1e1e1e".to_string()),
            ..Default::default()
        };
        let theme = Theme::from_config(Some(&config));
        assert_eq!(theme.accent, Color::Rgb(255, 87, 51));
        assert_eq!(theme.background, Color::Rgb(30, 30, 30));
        assert_eq!(theme.error, Color::Red); // unchanged
    }

    #[test]
    fn theme_from_config_ignores_invalid_colors() {
        let config = ThemeConfig {
            accent: Some("not-a-color".to_string()),
            ..Default::default()
        };
        let theme = Theme::from_config(Some(&config));
        assert_eq!(theme.accent, Color::Cyan);
    }

    #[test]
    fn theme_from_config_defaults_when_missing() {
        let theme = Theme::from_config(None);
        assert_eq!(theme.background, Color::Reset);
        assert_eq!(theme.accent, Color::Cyan);
    }
}
