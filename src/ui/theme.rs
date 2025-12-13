use ratatui::style::{Color, Modifier, Style};

use crate::app::state::ToastLevel;

/// Catppuccin Mocha theme
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub accent_secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub border: Color,
    pub border_focused: Color,
    pub selection_bg: Color,
    pub muted: Color,
    pub surface: Color,
}

pub static THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(|| Theme {
    bg: Color::Rgb(30, 30, 46),
    fg: Color::Rgb(205, 214, 244),
    accent: Color::Rgb(137, 180, 250),
    accent_secondary: Color::Rgb(180, 190, 254),
    success: Color::Rgb(166, 227, 161),
    warning: Color::Rgb(249, 226, 175),
    error: Color::Rgb(243, 139, 168),
    info: Color::Rgb(137, 220, 235),
    border: Color::Rgb(88, 91, 112),
    border_focused: Color::Rgb(137, 180, 250),
    selection_bg: Color::Rgb(69, 71, 90),
    muted: Color::Rgb(108, 112, 134),
    surface: Color::Rgb(49, 50, 68),
});

impl Theme {
    pub fn header_style(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    pub fn title_style(&self) -> Style {
        Style::default().fg(self.fg).add_modifier(Modifier::BOLD)
    }

    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.fg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default().bg(self.selection_bg).fg(self.fg).add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self, focused: bool) -> Style {
        Style::default().fg(if focused { self.border_focused } else { self.border })
    }

    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn loading_style(&self) -> Style {
        Style::default().fg(self.warning).add_modifier(Modifier::ITALIC)
    }

    pub fn table_header_style(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    pub fn modal_style(&self) -> Style {
        Style::default().bg(self.surface).fg(self.fg)
    }

    pub fn input_style(&self, focused: bool) -> Style {
        Style::default().fg(if focused { self.fg } else { self.muted }).bg(self.surface)
    }

    pub fn success_style(&self) -> Style { Style::default().fg(self.success) }
    pub fn warning_style(&self) -> Style { Style::default().fg(self.warning) }
    pub fn error_style(&self) -> Style { Style::default().fg(self.error) }
    pub fn info_style(&self) -> Style { Style::default().fg(self.info) }

    pub fn status_connected(&self) -> Style {
        Style::default().fg(self.success).add_modifier(Modifier::BOLD)
    }

    pub fn status_disconnected(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn status_connecting(&self) -> Style {
        Style::default().fg(self.warning).add_modifier(Modifier::ITALIC)
    }

    pub fn toast_style(&self, level: &ToastLevel) -> Style {
        Style::default().fg(match level {
            ToastLevel::Info => self.info,
            ToastLevel::Success => self.success,
            ToastLevel::Warning => self.warning,
            ToastLevel::Error => self.error,
        })
    }

    pub fn partition_style(&self) -> Style { Style::default().fg(self.info) }
    pub fn offset_style(&self) -> Style { Style::default().fg(self.accent_secondary) }

    pub fn lag_style(&self, lag: i64) -> Style {
        Style::default().fg(match lag {
            0 => self.success,
            1..=999 => self.warning,
            _ => self.error,
        })
    }

    pub fn consumer_group_state_style(&self, state: &str) -> Style {
        Style::default().fg(match state.to_lowercase().as_str() {
            "stable" => self.success,
            "empty" => self.muted,
            "dead" => self.error,
            "preparingrebalance" | "completingrebalance" => self.warning,
            _ => self.fg,
        })
    }

    pub fn sidebar_item_style(&self, selected: bool, focused: bool) -> Style {
        match (selected, focused) {
            (true, true) => Style::default().bg(self.accent).fg(self.bg).add_modifier(Modifier::BOLD),
            (true, false) => Style::default().bg(self.selection_bg).fg(self.fg),
            _ => Style::default().fg(self.fg),
        }
    }

    pub fn key_hint_style(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    pub fn key_desc_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn topic_internal_style(&self) -> Style {
        Style::default().fg(self.muted).add_modifier(Modifier::ITALIC)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default().bg(self.accent).fg(self.bg).add_modifier(Modifier::BOLD)
    }
}
