use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::state::PurgeTopicFormState;
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct PurgeTopicFormModal;

impl PurgeTopicFormModal {
    pub fn render(frame: &mut Frame, form_state: &PurgeTopicFormState) {
        let area = centered_rect_fixed(55, 12, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(" Purge Topic ")
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
                Constraint::Length(1), // Topic info
                Constraint::Length(1), // Warning
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Purge all checkbox
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Offset label
                Constraint::Length(1), // Offset input
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        // Topic name
        let topic_info = Paragraph::new(format!("Topic: {}", form_state.topic))
            .style(THEME.title_style());
        frame.render_widget(topic_info, chunks[0]);

        // Warning
        let warning = Paragraph::new("WARNING: This will permanently delete messages!")
            .style(Style::default().fg(THEME.error).add_modifier(Modifier::BOLD));
        frame.render_widget(warning, chunks[1]);

        // Purge all checkbox
        let checkbox = if form_state.purge_all { "[x]" } else { "[ ]" };
        let purge_all = Paragraph::new(format!("{} Purge all messages", checkbox))
            .style(THEME.normal_style());
        frame.render_widget(purge_all, chunks[3]);

        // Offset label and input (only if not purge_all)
        if !form_state.purge_all {
            let label = Paragraph::new("Delete messages before offset:")
                .style(THEME.muted_style());
            frame.render_widget(label, chunks[5]);

            let display = if form_state.offset.is_empty() {
                String::from("█")
            } else {
                format!("{}█", form_state.offset)
            };
            let input = Paragraph::new(display).style(THEME.input_style(true));
            frame.render_widget(input, chunks[6]);
        }

        // Hint
        let hint = Paragraph::new("Tab/↑/↓: toggle | Enter: purge | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[7]);
    }
}
