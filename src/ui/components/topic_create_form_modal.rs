use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::{TopicCreateFormField, TopicCreateFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct TopicCreateFormModal;

impl TopicCreateFormModal {
    pub fn render(frame: &mut Frame, form_state: &TopicCreateFormState) {
        let area = centered_rect_fixed(60, 15, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(" Create Topic ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true))
            .style(THEME.modal_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Form fields layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Name label
                Constraint::Length(1), // Name input
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Partitions label
                Constraint::Length(1), // Partitions input
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Replication label
                Constraint::Length(1), // Replication input
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        // Name label
        let name_focused = form_state.focused_field == TopicCreateFormField::Name;
        let name_label_style = if name_focused {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let name_label = Paragraph::new("Topic Name:").style(name_label_style);
        frame.render_widget(name_label, chunks[0]);

        // Name input
        let name_display = if form_state.name.is_empty() && name_focused {
            String::from("█")
        } else if name_focused {
            format!("{}█", form_state.name)
        } else if form_state.name.is_empty() {
            String::from("(required)")
        } else {
            form_state.name.clone()
        };
        let name_input = Paragraph::new(name_display)
            .style(THEME.input_style(name_focused));
        frame.render_widget(name_input, chunks[1]);

        // Partitions label
        let partitions_focused = form_state.focused_field == TopicCreateFormField::Partitions;
        let partitions_label_style = if partitions_focused {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let partitions_label = Paragraph::new("Partitions:").style(partitions_label_style);
        frame.render_widget(partitions_label, chunks[3]);

        // Partitions input
        let partitions_display = if form_state.partitions.is_empty() && partitions_focused {
            String::from("█")
        } else if partitions_focused {
            format!("{}█", form_state.partitions)
        } else if form_state.partitions.is_empty() {
            String::from("1")
        } else {
            form_state.partitions.clone()
        };
        let partitions_input = Paragraph::new(partitions_display)
            .style(THEME.input_style(partitions_focused));
        frame.render_widget(partitions_input, chunks[4]);

        // Replication factor label
        let replication_focused = form_state.focused_field == TopicCreateFormField::ReplicationFactor;
        let replication_label_style = if replication_focused {
            THEME.title_style()
        } else {
            THEME.muted_style()
        };
        let replication_label = Paragraph::new("Replication Factor:").style(replication_label_style);
        frame.render_widget(replication_label, chunks[6]);

        // Replication input
        let replication_display = if form_state.replication_factor.is_empty() && replication_focused {
            String::from("█")
        } else if replication_focused {
            format!("{}█", form_state.replication_factor)
        } else if form_state.replication_factor.is_empty() {
            String::from("1")
        } else {
            form_state.replication_factor.clone()
        };
        let replication_input = Paragraph::new(replication_display)
            .style(THEME.input_style(replication_focused));
        frame.render_widget(replication_input, chunks[7]);

        // Hint
        let hint = Paragraph::new("Tab: switch field | Enter: create | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[9]);
    }
}
