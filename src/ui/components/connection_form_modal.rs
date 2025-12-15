use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::app::state::{ConnectionFormField, ConnectionFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::{format_input, label_style, modal_block};

pub struct ConnectionFormModal;

impl ConnectionFormModal {
    pub fn render(frame: &mut Frame, form_state: &ConnectionFormState) {
        let height = if form_state.auth_type.requires_credentials() { 24 } else { 18 };
        let area = centered_rect_fixed(60, height, frame.area());

        frame.render_widget(Clear, area);

        let block = modal_block("New Connection");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut constraints = vec![
            Constraint::Length(1), // Name label
            Constraint::Length(1), // Name input
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Brokers label
            Constraint::Length(1), // Brokers input
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Consumer Group label
            Constraint::Length(1), // Consumer Group input
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Auth type label
            Constraint::Length(1), // Auth type selector
        ];

        if form_state.auth_type.requires_credentials() {
            constraints.extend([
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Username label
                Constraint::Length(1), // Username input
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Password label
                Constraint::Length(1), // Password input
            ]);
        }

        constraints.push(Constraint::Length(1)); // Spacer
        constraints.push(Constraint::Length(1)); // Hint

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(inner);

        let mut idx = 0;

        // Name field
        let name_focused = form_state.focused_field == ConnectionFormField::Name;
        Self::render_field(frame, &chunks, &mut idx, "Connection Name:", &form_state.name, "(empty)", name_focused);
        idx += 1; // skip spacer

        // Brokers field
        let brokers_focused = form_state.focused_field == ConnectionFormField::Brokers;
        Self::render_field(frame, &chunks, &mut idx, "Bootstrap Servers:", &form_state.brokers, "localhost:9092", brokers_focused);
        idx += 1; // skip spacer

        // Consumer Group field
        let cg_focused = form_state.focused_field == ConnectionFormField::ConsumerGroup;
        Self::render_field(frame, &chunks, &mut idx, "Consumer Group (optional):", &form_state.consumer_group, "kafka-tui", cg_focused);
        idx += 1; // skip spacer

        // Auth type selector
        let auth_focused = form_state.focused_field == ConnectionFormField::AuthType;
        let auth_label = Paragraph::new("Authentication:").style(label_style(auth_focused));
        frame.render_widget(auth_label, chunks[idx]);
        idx += 1;

        let auth_display = format!("◀ {} ▶", form_state.auth_type.display_name());
        let auth_input = Paragraph::new(auth_display).style(THEME.input_style(auth_focused));
        frame.render_widget(auth_input, chunks[idx]);
        idx += 1;

        // Credentials fields (if needed)
        if form_state.auth_type.requires_credentials() {
            idx += 1; // skip spacer

            let user_focused = form_state.focused_field == ConnectionFormField::Username;
            Self::render_field(frame, &chunks, &mut idx, "Username:", &form_state.username, "(empty)", user_focused);
            idx += 1; // skip spacer

            let pass_focused = form_state.focused_field == ConnectionFormField::Password;
            let pass_masked = "*".repeat(form_state.password.len());
            Self::render_field(frame, &chunks, &mut idx, "Password:", &pass_masked, "(empty)", pass_focused);
        }

        idx += 1; // skip spacer

        let hint_text = if form_state.focused_field == ConnectionFormField::AuthType {
            "←/→: change auth | Tab: next | Enter: connect | Esc: cancel"
        } else {
            "Tab: next field | Enter: connect | Esc: cancel"
        };
        let hint = Paragraph::new(hint_text)
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[idx]);
    }

    fn render_field(
        frame: &mut Frame,
        chunks: &std::rc::Rc<[Rect]>,
        idx: &mut usize,
        label_text: &str,
        value: &str,
        placeholder: &str,
        focused: bool,
    ) {
        let label = Paragraph::new(label_text).style(label_style(focused));
        frame.render_widget(label, chunks[*idx]);
        *idx += 1;

        let display = format_input(value, focused, placeholder);
        let input = Paragraph::new(display).style(THEME.input_style(focused));
        frame.render_widget(input, chunks[*idx]);
        *idx += 1;
    }
}
