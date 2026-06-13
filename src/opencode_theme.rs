use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ratatui::style::Color;
use serde_json::Value;

use crate::tui::theme::parse_hex_color;

const BUILTIN_THEME_URL: &str =
    "https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/tui/src/theme/assets";

/// A subset of an opencode theme resolved to `ratatui` colors.
///
/// Only the fields that map to `opencode-selector`'s internal palette are kept;
/// everything else is ignored. `None` means "fall back to the terminal-adaptive
/// default".
#[derive(Debug, Default, Clone, Copy)]
pub struct OpencodeTheme {
    pub background: Option<Color>,
    pub foreground: Option<Color>,
    pub accent: Option<Color>,
    pub accent_dim: Option<Color>,
    pub border: Option<Color>,
    pub highlight: Option<Color>,
    pub highlight_dim: Option<Color>,
    pub error: Option<Color>,
    pub warning: Option<Color>,
    pub success: Option<Color>,
    pub muted: Option<Color>,
}

/// Load the active opencode theme by reading `tui.json` and resolving the
/// referenced theme file.
///
/// The lookup order is:
/// 1. Custom theme files on disk (`~/.config/opencode/themes`, project-root,
///    cwd).
/// 2. Built-in opencode themes downloaded from GitHub and cached under
///    `cache_dir/themes`.
///
/// Returns `None` when the theme cannot be resolved or loaded.
pub fn load_active_theme(
    opencode_config_dir: &Path,
    cwd: Option<&Path>,
    cache_dir: Option<&Path>,
) -> Option<OpencodeTheme> {
    let tui = resolve_tui_json(opencode_config_dir, cwd)?;
    let theme_name = tui
        .get("theme")
        .and_then(Value::as_str)
        .unwrap_or("opencode");

    // The "system" theme is generated at runtime and has no JSON definition.
    if theme_name == "system" {
        return None;
    }

    if let Some(path) = find_theme_file(opencode_config_dir, cwd, theme_name) {
        return load_theme_from_path(&path);
    }

    if let Some(cache) = cache_dir {
        if let Some(path) = fetch_builtin_theme(theme_name, cache) {
            return load_theme_from_path(&path);
        }
    }

    None
}

fn load_theme_from_path(path: &Path) -> Option<OpencodeTheme> {
    let content = std::fs::read_to_string(path).ok()?;
    let value: Value = serde_json::from_str(&content).ok()?;
    Some(resolve_opencode_theme(&value))
}

fn fetch_builtin_theme(name: &str, cache_dir: &Path) -> Option<PathBuf> {
    let themes_dir = cache_dir.join("themes");
    let cache_path = themes_dir.join(format!("{name}.json"));

    if cache_path.is_file() {
        return Some(cache_path);
    }

    let url = format!("{BUILTIN_THEME_URL}/{name}.json");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let response = client.get(&url).send().ok()?.error_for_status().ok()?;
    let content = response.text().ok()?;

    std::fs::create_dir_all(&themes_dir).ok()?;
    std::fs::write(&cache_path, content).ok()?;
    Some(cache_path)
}

fn resolve_tui_json(opencode_config_dir: &Path, cwd: Option<&Path>) -> Option<Value> {
    if let Some(cwd) = cwd {
        let local = cwd.join(".opencode").join("tui.json");
        if local.is_file() {
            return serde_json::from_str(&std::fs::read_to_string(local).ok()?).ok();
        }
    }

    let global = opencode_config_dir.join("tui.json");
    if global.is_file() {
        return serde_json::from_str(&std::fs::read_to_string(global).ok()?).ok();
    }

    None
}

fn find_theme_file(opencode_config_dir: &Path, cwd: Option<&Path>, name: &str) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(
        opencode_config_dir
            .join("themes")
            .join(format!("{name}.json")),
    );

    if let Some(cwd) = cwd {
        if let Some(root) = project_root(cwd) {
            candidates.push(
                root.join(".opencode")
                    .join("themes")
                    .join(format!("{name}.json")),
            );
        }
        candidates.push(
            cwd.join(".opencode")
                .join("themes")
                .join(format!("{name}.json")),
        );
    }

    candidates.into_iter().find(|p| p.is_file())
}

fn project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start;
    loop {
        if current.join(".git").exists() {
            return Some(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

fn resolve_opencode_theme(value: &Value) -> OpencodeTheme {
    let defs = resolve_defs(value.get("defs"));
    let theme = value.get("theme");

    OpencodeTheme {
        background: color(theme, "background", &defs),
        foreground: color(theme, "text", &defs),
        accent: color(theme, "accent", &defs),
        accent_dim: color(theme, "textMuted", &defs),
        border: color(theme, "border", &defs),
        highlight: color(theme, "primary", &defs),
        highlight_dim: color(theme, "secondary", &defs),
        error: color(theme, "error", &defs),
        warning: color(theme, "warning", &defs),
        success: color(theme, "success", &defs),
        muted: color(theme, "textMuted", &defs),
    }
}

fn resolve_defs(defs: Option<&Value>) -> HashMap<String, Color> {
    let mut map = HashMap::new();
    let Some(Value::Object(obj)) = defs else {
        return map;
    };

    // Allow defs to reference other defs with a few fixed-point passes.
    for _ in 0..obj.len() {
        let mut changed = false;
        for (key, value) in obj {
            if map.contains_key(key) {
                continue;
            }
            if let Some(c) = resolve_value(value, &map, "dark") {
                map.insert(key.clone(), c);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    map
}

fn color(theme: Option<&Value>, key: &str, defs: &HashMap<String, Color>) -> Option<Color> {
    theme
        .and_then(|t| t.get(key))
        .and_then(|v| resolve_value(v, defs, "dark"))
}

fn resolve_value(value: &Value, defs: &HashMap<String, Color>, variant: &str) -> Option<Color> {
    match value {
        Value::String(s) => resolve_string(s, defs, variant),
        Value::Object(map) => map
            .get(variant)
            .and_then(|v| resolve_value(v, defs, variant)),
        Value::Number(n) => n
            .as_u64()
            .and_then(|v| (v <= 255).then_some(Color::Indexed(v as u8))),
        _ => None,
    }
}

fn resolve_string(value: &str, defs: &HashMap<String, Color>, _variant: &str) -> Option<Color> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("none") {
        return Some(Color::Reset);
    }
    if value.starts_with('#') {
        return parse_hex_color(value);
    }
    if let Ok(index) = value.parse::<u8>() {
        return Some(Color::Indexed(index));
    }
    defs.get(value).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_opencode_theme_reads_defs_and_dark_variant() {
        let json = serde_json::json!({
            "defs": {
                "bg": "#1e1e1e",
                "fg": "#e6edf3",
                "blue": { "dark": "#2f81f7", "light": "#0969da" }
            },
            "theme": {
                "background": "bg",
                "text": "fg",
                "primary": "blue",
                "textMuted": "#7d8590"
            }
        });

        let theme = resolve_opencode_theme(&json);
        assert_eq!(theme.background, Some(Color::Rgb(30, 30, 30)));
        assert_eq!(theme.foreground, Some(Color::Rgb(230, 237, 243)));
        assert_eq!(theme.highlight, Some(Color::Rgb(47, 129, 247)));
        assert_eq!(theme.muted, Some(Color::Rgb(125, 133, 144)));
    }

    #[test]
    fn resolve_string_handles_none_and_hex() {
        let defs = HashMap::new();
        assert_eq!(resolve_string("none", &defs, "dark"), Some(Color::Reset));
        assert_eq!(
            resolve_string("#ff5733", &defs, "dark"),
            Some(Color::Rgb(255, 87, 51))
        );
        assert_eq!(resolve_string("invalid", &defs, "dark"), None);
    }

    #[test]
    fn resolve_string_uses_defs() {
        let mut defs = HashMap::new();
        defs.insert("myblue".to_string(), Color::Rgb(0, 0, 255));
        assert_eq!(
            resolve_string("myblue", &defs, "dark"),
            Some(Color::Rgb(0, 0, 255))
        );
    }

    #[test]
    fn project_root_finds_git_directory() {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path().join("repo");
        std::fs::create_dir(&repo).unwrap();
        std::fs::create_dir(repo.join(".git")).unwrap();
        let nested = repo.join("src").join("nested");
        std::fs::create_dir_all(&nested).unwrap();

        assert_eq!(project_root(&nested), Some(repo.clone()));
        assert_eq!(project_root(&repo), Some(repo));
        assert_eq!(project_root(temp.path()), None);
    }

    #[test]
    fn load_active_theme_reads_local_theme_file() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("opencode");
        let themes_dir = config_dir.join("themes");
        std::fs::create_dir_all(&themes_dir).unwrap();

        let tui_json = config_dir.join("tui.json");
        std::fs::write(
            &tui_json,
            serde_json::json!({ "theme": "custom" }).to_string(),
        )
        .unwrap();

        let theme_json = themes_dir.join("custom.json");
        std::fs::write(
            &theme_json,
            serde_json::json!({
                "theme": {
                    "background": "#111111",
                    "text": "#eeeeee",
                    "primary": "#ff0000"
                }
            })
            .to_string(),
        )
        .unwrap();

        let theme = load_active_theme(&config_dir, None, None).unwrap();
        assert_eq!(theme.background, Some(Color::Rgb(17, 17, 17)));
        assert_eq!(theme.foreground, Some(Color::Rgb(238, 238, 238)));
        assert_eq!(theme.highlight, Some(Color::Rgb(255, 0, 0)));
    }
}
