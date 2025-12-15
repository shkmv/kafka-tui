use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::app::state::PurgeTopicFormState;
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;
use crate::ui::widgets::{format_input, modal_block};

pub struct PurgeTopicFormModal;

impl PurgeTopicFormModal {
    pub fn render(frame: &mut Frame, form_state: &PurgeTopicFormState) {
        let area = centered_rect_fixed(55, 12, frame.area());

        frame.render_widget(Clear, area);

        let block = modal_block("Purge Topic");
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

        let topic_info = Paragraph::new(format!("Topic: {}", form_state.topic))
            .style(THEME.title_style());
        frame.render_widget(topic_info, chunks[0]);

        let warning = Paragraph::new("WARNING: This will permanently delete messages!")
            .style(Style::default().fg(THEME.error).add_modifier(Modifier::BOLD));
        frame.render_widget(warning, chunks[1]);

        let checkbox = if form_state.purge_all { "[x]" } else { "[ ]" };
        let purge_all = Paragraph::new(format!("{} Purge all messages", checkbox))
            .style(THEME.normal_style());
        frame.render_widget(purge_all, chunks[3]);

        if !form_state.purge_all {
            let label = Paragraph::new("Delete messages before offset:")
                .style(THEME.muted_style());
            frame.render_widget(label, chunks[5]);

            let display = format_input(&form_state.offset, true, "");
            let input = Paragraph::new(display).style(THEME.input_style(true));
            frame.render_widget(input, chunks[6]);
        }

        let hint = Paragraph::new("Tab/Space: toggle | Enter: purge | Esc: cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[7]);
    }
}
