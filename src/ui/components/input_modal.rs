use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct InputModal;

impl InputModal {
    pub fn render(
        frame: &mut Frame,
        title: &str,
        placeholder: &str,
        value: &str,
    ) {
        let area = centered_rect_fixed(50, 7, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!(" {} ", title))
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Input field layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Input
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        // Placeholder/label
        let label = if value.is_empty() {
            Paragraph::new(placeholder).style(THEME.muted_style())
        } else {
            Paragraph::new("").style(THEME.muted_style())
        };
        frame.render_widget(label, chunks[0]);

        // Input value with cursor
        let display_value = if value.is_empty() {
            String::from("█")
        } else {
            format!("{}█", value)
        };
        let input = Paragraph::new(display_value)
            .style(THEME.input_style(true));
        frame.render_widget(input, chunks[1]);

        // Hint
        let hint = Paragraph::new("Enter to confirm, Esc to cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[2]);
    }
}
