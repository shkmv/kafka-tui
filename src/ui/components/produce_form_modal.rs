use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::{ProduceFormField, ProduceFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::render_labeled_input;

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

        let key_focused = form_state.focused_field == ProduceFormField::Key;
        render_labeled_input(
            frame, chunks[0], chunks[1],
            "Key (optional):", &form_state.key, "(null)", key_focused,
        );

        let value_focused = form_state.focused_field == ProduceFormField::Value;
        render_labeled_input(
            frame, chunks[3], chunks[4],
            "Value:", &form_state.value, "(required)", value_focused,
        );

        let hint = Paragraph::new("Tab: switch field | Enter: send | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[5]);
    }
}
