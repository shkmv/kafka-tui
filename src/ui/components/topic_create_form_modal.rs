use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::app::state::{TopicCreateFormField, TopicCreateFormState};
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::{modal_block, render_labeled_input};

pub struct TopicCreateFormModal;

impl TopicCreateFormModal {
    pub fn render(frame: &mut Frame, form_state: &TopicCreateFormState) {
        let area = centered_rect_fixed(60, 15, frame.area());

        frame.render_widget(Clear, area);

        let block = modal_block("Create Topic");
        let inner = block.inner(area);
        frame.render_widget(block, area);

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

        let name_focused = form_state.focused_field == TopicCreateFormField::Name;
        render_labeled_input(
            frame, chunks[0], chunks[1],
            "Topic Name:", &form_state.name, "(required)", name_focused,
        );

        let partitions_focused = form_state.focused_field == TopicCreateFormField::Partitions;
        render_labeled_input(
            frame, chunks[3], chunks[4],
            "Partitions:", &form_state.partitions, "1", partitions_focused,
        );

        let replication_focused = form_state.focused_field == TopicCreateFormField::ReplicationFactor;
        render_labeled_input(
            frame, chunks[6], chunks[7],
            "Replication Factor:", &form_state.replication_factor, "1", replication_focused,
        );

        let hint = Paragraph::new("Tab: switch field | Enter: create | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[9]);
    }
}
