use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct ConfirmModal;

impl ConfirmModal {
    pub fn render(
        frame: &mut Frame,
        title: &str,
        message: &str,
    ) {
        let area = centered_rect_fixed(50, 9, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!(" {} ", title))
            .title_style(THEME.warning_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Content layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(2),    // Message
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Buttons hint
            ])
            .split(inner);

        // Message
        let message_widget = Paragraph::new(message)
            .style(THEME.normal_style())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        frame.render_widget(message_widget, chunks[0]);

        // Button hints
        let hints = Line::from(vec![
            Span::styled("[Y]", THEME.key_hint_style()),
            Span::styled(" Yes  ", THEME.normal_style()),
            Span::styled("[N]", THEME.key_hint_style()),
            Span::styled(" No", THEME.normal_style()),
        ]);
        let hints_widget = Paragraph::new(hints)
            .alignment(Alignment::Center);
        frame.render_widget(hints_widget, chunks[2]);
    }
}
