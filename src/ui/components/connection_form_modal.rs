use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::{ConnectionFormField, ConnectionFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct ConnectionFormModal;

impl ConnectionFormModal {
    pub fn render(frame: &mut Frame, form_state: &ConnectionFormState) {
        // Dynamic height based on whether credentials are needed
        let height = if form_state.auth_type.requires_credentials() {
            24
        } else {
            18
        };
        let area = centered_rect_fixed(60, height, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(" New Connection ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Build constraints dynamically
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

        // Name label
        let name_label_style = if form_state.focused_field == ConnectionFormField::Name {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let name_label = Paragraph::new("Connection Name:").style(name_label_style);
        frame.render_widget(name_label, chunks[idx]);
        idx += 1;

        // Name input
        let name_focused = form_state.focused_field == ConnectionFormField::Name;
        let name_display = Self::format_input(&form_state.name, name_focused, "(empty)");
        let name_input = Paragraph::new(name_display).style(THEME.input_style(name_focused));
        frame.render_widget(name_input, chunks[idx]);
        idx += 2; // skip spacer

        // Brokers label
        let brokers_label_style = if form_state.focused_field == ConnectionFormField::Brokers {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let brokers_label = Paragraph::new("Bootstrap Servers:").style(brokers_label_style);
        frame.render_widget(brokers_label, chunks[idx]);
        idx += 1;

        // Brokers input
        let brokers_focused = form_state.focused_field == ConnectionFormField::Brokers;
        let brokers_display = Self::format_input(&form_state.brokers, brokers_focused, "localhost:9092");
        let brokers_input = Paragraph::new(brokers_display).style(THEME.input_style(brokers_focused));
        frame.render_widget(brokers_input, chunks[idx]);
        idx += 2; // skip spacer

        // Consumer Group label
        let cg_label_style = if form_state.focused_field == ConnectionFormField::ConsumerGroup {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let cg_label = Paragraph::new("Consumer Group (optional):").style(cg_label_style);
        frame.render_widget(cg_label, chunks[idx]);
        idx += 1;

        // Consumer Group input
        let cg_focused = form_state.focused_field == ConnectionFormField::ConsumerGroup;
        let cg_display = Self::format_input(&form_state.consumer_group, cg_focused, "kafka-tui");
        let cg_input = Paragraph::new(cg_display).style(THEME.input_style(cg_focused));
        frame.render_widget(cg_input, chunks[idx]);
        idx += 2; // skip spacer

        // Auth type label
        let auth_label_style = if form_state.focused_field == ConnectionFormField::AuthType {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let auth_label = Paragraph::new("Authentication:").style(auth_label_style);
        frame.render_widget(auth_label, chunks[idx]);
        idx += 1;

        // Auth type selector
        let auth_focused = form_state.focused_field == ConnectionFormField::AuthType;
        let auth_display = format!(
            "◀ {} ▶",
            form_state.auth_type.display_name()
        );
        let auth_style = if auth_focused {
            THEME.input_style(true)
        } else {
            THEME.input_style(false)
        };
        let auth_input = Paragraph::new(auth_display).style(auth_style);
        frame.render_widget(auth_input, chunks[idx]);
        idx += 1;

        // Credentials fields (if needed)
        if form_state.auth_type.requires_credentials() {
            idx += 1; // skip spacer

            // Username label
            let user_label_style = if form_state.focused_field == ConnectionFormField::Username {
                THEME.title_style()
            } else {
                THEME.muted_style()
            };
            let user_label = Paragraph::new("Username:").style(user_label_style);
            frame.render_widget(user_label, chunks[idx]);
            idx += 1;

            // Username input
            let user_focused = form_state.focused_field == ConnectionFormField::Username;
            let user_display = Self::format_input(&form_state.username, user_focused, "(empty)");
            let user_input = Paragraph::new(user_display).style(THEME.input_style(user_focused));
            frame.render_widget(user_input, chunks[idx]);
            idx += 2; // skip spacer

            // Password label
            let pass_label_style = if form_state.focused_field == ConnectionFormField::Password {
                THEME.title_style()
            } else {
                THEME.muted_style()
            };
            let pass_label = Paragraph::new("Password:").style(pass_label_style);
            frame.render_widget(pass_label, chunks[idx]);
            idx += 1;

            // Password input (masked)
            let pass_focused = form_state.focused_field == ConnectionFormField::Password;
            let pass_masked = "*".repeat(form_state.password.len());
            let pass_display = Self::format_input(&pass_masked, pass_focused, "(empty)");
            let pass_input = Paragraph::new(pass_display).style(THEME.input_style(pass_focused));
            frame.render_widget(pass_input, chunks[idx]);
            idx += 1;
        }

        idx += 1; // skip spacer

        // Hint
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

    fn format_input(value: &str, focused: bool, placeholder: &str) -> String {
        if value.is_empty() && focused {
            String::from("█")
        } else if focused {
            format!("{}█", value)
        } else if value.is_empty() {
            String::from(placeholder)
        } else {
            value.to_string()
        }
    }
}
