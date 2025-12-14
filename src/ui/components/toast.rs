use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::{Level, ToastMessage};
use crate::ui::theme::THEME;

pub struct Toast;

impl Toast {
    pub fn render(frame: &mut Frame, toasts: &[ToastMessage]) {
        if toasts.is_empty() {
            return;
        }

        let frame_area = frame.area();

        // Render toasts from bottom-right corner, stacking upward
        let toast_width = 60u16.min(frame_area.width.saturating_sub(4));
        let toast_height = 3u16;
        let margin = 2u16;

        for (i, toast) in toasts.iter().rev().take(3).enumerate() {
            let y_offset = (i as u16) * (toast_height + 1) + margin;

            if frame_area.height < y_offset + toast_height + margin {
                continue;
            }

            let area = Rect::new(
                frame_area.width.saturating_sub(toast_width + margin),
                frame_area.height.saturating_sub(y_offset + toast_height + margin),
                toast_width,
                toast_height,
            );

            Self::render_single(frame, area, toast);
        }
    }

    fn render_single(frame: &mut Frame, area: Rect, toast: &ToastMessage) {
        // Clear background
        frame.render_widget(Clear, area);

        let (icon, border_style) = match toast.level {
            Level::Info => ("", THEME.info_style()),
            Level::Success => ("", THEME.success_style()),
            Level::Warning => ("", THEME.warning_style()),
            Level::Error => ("", THEME.error_style()),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Message with icon
        let message = format!(" {} {}", icon, toast.message);
        let truncated = if message.len() > (inner.width as usize - 2) {
            format!("{}...", &message[..inner.width as usize - 5])
        } else {
            message
        };

        let paragraph = Paragraph::new(truncated)
            .style(THEME.toast_style(&toast.level));
        frame.render_widget(paragraph, inner);
    }
}
