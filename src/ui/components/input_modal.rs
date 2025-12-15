use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::{format_input, modal_block};

pub struct InputModal;

impl InputModal {
    pub fn render(
        frame: &mut Frame,
        title: &str,
        placeholder: &str,
        value: &str,
    ) {
        let area = centered_rect_fixed(50, 7, frame.area());

        frame.render_widget(Clear, area);

        let block = modal_block(title);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Input
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        let label = if value.is_empty() {
            Paragraph::new(placeholder).style(THEME.muted_style())
        } else {
            Paragraph::new("").style(THEME.muted_style())
        };
        frame.render_widget(label, chunks[0]);

        let display_value = format_input(value, true, "");
        let input = Paragraph::new(display_value).style(THEME.input_style(true));
        frame.render_widget(input, chunks[1]);

        let hint = Paragraph::new("Enter to confirm, Esc to cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[2]);
    }
}
