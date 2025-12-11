use ratatui::style::{Color, Modifier, Style};

use crate::app::state::ToastLevel;

/// Catppuccin Mocha theme colors
pub struct Theme {
    // Base colors
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub accent_secondary: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Component colors
    pub border: Color,
    pub border_focused: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub muted: Color,
    pub surface: Color,
    pub overlay: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl Theme {
    /// Catppuccin Mocha theme
    pub fn catppuccin_mocha() -> Self {
        Self {
            // Base colors
            bg: Color::Rgb(30, 30, 46),         // Base
            fg: Color::Rgb(205, 214, 244),      // Text
            accent: Color::Rgb(137, 180, 250),  // Blue
            accent_secondary: Color::Rgb(180, 190, 254), // Lavender

            // Semantic colors
            success: Color::Rgb(166, 227, 161), // Green
            warning: Color::Rgb(249, 226, 175), // Yellow
            error: Color::Rgb(243, 139, 168),   // Red (Maroon)
            info: Color::Rgb(137, 220, 235),    // Teal

            // Component colors
            border: Color::Rgb(88, 91, 112),    // Surface2
            border_focused: Color::Rgb(137, 180, 250), // Blue
            selection_bg: Color::Rgb(69, 71, 90), // Surface1
            selection_fg: Color::Rgb(205, 214, 244), // Text
            muted: Color::Rgb(108, 112, 134),   // Overlay0
            surface: Color::Rgb(49, 50, 68),    // Surface0
            overlay: Color::Rgb(108, 112, 134), // Overlay0
        }
    }

    // === Style Builders ===

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.fg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .bg(self.selection_bg)
            .fg(self.selection_fg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .bg(self.accent)
            .fg(self.bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self, focused: bool) -> Style {
        if focused {
            Style::default().fg(self.border_focused)
        } else {
            Style::default().fg(self.border)
        }
    }

    pub fn status_connected(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_disconnected(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn status_connecting(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn key_hint_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_desc_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn toast_style(&self, level: &ToastLevel) -> Style {
        let color = match level {
            ToastLevel::Info => self.info,
            ToastLevel::Success => self.success,
            ToastLevel::Warning => self.warning,
            ToastLevel::Error => self.error,
        };
        Style::default().fg(color)
    }

    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    pub fn info_style(&self) -> Style {
        Style::default().fg(self.info)
    }

    pub fn loading_style(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn table_header_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn sidebar_item_style(&self, selected: bool, focused: bool) -> Style {
        if selected && focused {
            Style::default()
                .bg(self.accent)
                .fg(self.bg)
                .add_modifier(Modifier::BOLD)
        } else if selected {
            Style::default()
                .bg(self.selection_bg)
                .fg(self.fg)
        } else {
            Style::default().fg(self.fg)
        }
    }

    pub fn modal_style(&self) -> Style {
        Style::default().bg(self.surface).fg(self.fg)
    }

    pub fn input_style(&self, focused: bool) -> Style {
        if focused {
            Style::default()
                .fg(self.fg)
                .bg(self.surface)
        } else {
            Style::default()
                .fg(self.muted)
                .bg(self.surface)
        }
    }

    // === Specific Component Styles ===

    pub fn topic_internal_style(&self) -> Style {
        Style::default()
            .fg(self.muted)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn partition_style(&self) -> Style {
        Style::default().fg(self.info)
    }

    pub fn offset_style(&self) -> Style {
        Style::default().fg(self.accent_secondary)
    }

    pub fn lag_style(&self, lag: i64) -> Style {
        if lag == 0 {
            Style::default().fg(self.success)
        } else if lag < 1000 {
            Style::default().fg(self.warning)
        } else {
            Style::default().fg(self.error)
        }
    }

    pub fn consumer_group_state_style(&self, state: &str) -> Style {
        match state.to_lowercase().as_str() {
            "stable" => Style::default().fg(self.success),
            "empty" => Style::default().fg(self.muted),
            "dead" => Style::default().fg(self.error),
            "preparingrebalance" | "completingrebalance" => Style::default().fg(self.warning),
            _ => Style::default().fg(self.fg),
        }
    }
}

/// Global theme instance
pub static THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(Theme::default);
