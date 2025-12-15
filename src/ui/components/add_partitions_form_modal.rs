use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::app::state::AddPartitionsFormState;
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::{format_input, modal_block};

pub struct AddPartitionsFormModal;

impl AddPartitionsFormModal {
    pub fn render(frame: &mut Frame, form_state: &AddPartitionsFormState) {
        let area = centered_rect_fixed(50, 10, frame.area());

        frame.render_widget(Clear, area);

        let block = modal_block("Add Partitions");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Topic info
                Constraint::Length(1), // Current count
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // New count label
                Constraint::Length(1), // New count input
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        let topic_info = Paragraph::new(format!("Topic: {}", form_state.topic))
            .style(THEME.title_style());
        frame.render_widget(topic_info, chunks[0]);

        let current = Paragraph::new(format!("Current partitions: {}", form_state.current_count))
            .style(THEME.muted_style());
        frame.render_widget(current, chunks[1]);

        let label = Paragraph::new("New partition count:").style(THEME.normal_style());
        frame.render_widget(label, chunks[3]);

        let display = format_input(&form_state.new_count, true, "");
        let input = Paragraph::new(display).style(THEME.input_style(true));
        frame.render_widget(input, chunks[4]);

        let hint = Paragraph::new("Enter: confirm | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[5]);
    }
}
