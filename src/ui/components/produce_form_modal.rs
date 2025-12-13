use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::{ProduceFormField, ProduceFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct ProduceFormModal;

impl ProduceFormModal {
    pub fn render(frame: &mut Frame, form_state: &ProduceFormState) {
        let area = centered_rect_fixed(60, 13, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!(" Produce to: {} ", form_state.topic))
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Key label
                Constraint::Length(1), // Key input
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Value label
                Constraint::Length(3), // Value input (multiline)
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        // Key label
        let key_focused = form_state.focused_field == ProduceFormField::Key;
        let key_label = Paragraph::new("Key (optional):")
            .style(if key_focused { THEME.title_style() } else { THEME.muted_style() });
        frame.render_widget(key_label, chunks[0]);

        // Key input
        let key_display = if form_state.key.is_empty() && key_focused {
            String::from("█")
        } else if key_focused {
            format!("{}█", form_state.key)
        } else if form_state.key.is_empty() {
            String::from("(null)")
        } else {
            form_state.key.clone()
        };
        let key_input = Paragraph::new(key_display)
            .style(THEME.input_style(key_focused));
        frame.render_widget(key_input, chunks[1]);

        // Value label
        let value_focused = form_state.focused_field == ProduceFormField::Value;
        let value_label = Paragraph::new("Value:")
            .style(if value_focused { THEME.title_style() } else { THEME.muted_style() });
        frame.render_widget(value_label, chunks[3]);

        // Value input
        let value_display = if form_state.value.is_empty() && value_focused {
            String::from("█")
        } else if value_focused {
            format!("{}█", form_state.value)
        } else if form_state.value.is_empty() {
            String::from("(required)")
        } else {
            form_state.value.clone()
        };
        let value_input = Paragraph::new(value_display)
            .style(THEME.input_style(value_focused));
        frame.render_widget(value_input, chunks[4]);

        // Hint
        let hint = Paragraph::new("Tab: switch field | Enter: send | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[5]);
    }
}
