---
name: opencode-selector-ui-review
description: Review Ratatui UI/UX changes in opencode-selector for consistency, accessibility, and keybindings.
---

# opencode-selector-ui-review

Use this skill when reviewing or improving the TUI of `opencode-selector`.

## Trigger

- A change touches `src/tui/`.
- User asks about colors, layout, keybindings, or accessibility.

## Checklist

1. **Colors**
   - Are colors terminal-adaptive (16-color palette)?
   - Are hardcoded theme colors avoided?

2. **Layout**
   - Does the layout handle small terminals gracefully?
   - Are borders and spacing consistent?

3. **Keybindings**
   - Are arrow keys, vim keys (`hjkl`), and mouse supported where applicable?
   - Is `Esc`/`q` used for back/quit?
   - Is `?` used for help?

4. **Feedback**
   - Does the status bar show current mode and shortcuts?
   - Are destructive actions confirmed?

5. **Accessibility**
   - Is selected state visually distinct?
   - Are text contrast and alignment readable?

## Constraints

- Do not introduce theme-specific color palettes (solarized, catppuccin, etc.).
- Keep preview as a full-screen modal, not a permanent panel.
