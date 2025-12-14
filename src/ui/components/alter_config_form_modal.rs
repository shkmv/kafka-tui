use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
};

use crate::app::state::AlterConfigFormState;
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::THEME;

pub struct AlterConfigFormModal;

impl AlterConfigFormModal {
    pub fn render(frame: &mut Frame, form_state: &AlterConfigFormState) {
        let area = centered_rect_fixed(70, 20, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!(" Edit Config: {} ", form_state.topic))
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
                Constraint::Min(5),    // Config table
                Constraint::Length(1), // Edit input (if editing)
                Constraint::Length(1), // Hint
            ])
            .split(inner);

        // Config table
        let rows: Vec<Row> = form_state
            .configs
            .iter()
            .enumerate()
            .map(|(i, (key, value, modified))| {
                let selected = i == form_state.selected_index;
                let style = if selected {
                    THEME.selected_style()
                } else if *modified {
                    Style::default().fg(THEME.accent)
                } else {
                    THEME.normal_style()
                };

                let marker = if *modified { "*" } else { " " };
                Row::new(vec![
                    format!("{}{}", marker, key),
                    value.clone(),
                ])
                .style(style)
            })
            .collect();

        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec!["Config Key", "Value"])
                    .style(THEME.header_style())
                    .bottom_margin(1),
            )
            .block(Block::default());

        frame.render_widget(table, chunks[0]);

        // Edit input (shown when editing)
        if form_state.editing {
            let edit_display = format!("New value: {}█", form_state.edit_value);
            let edit_input = Paragraph::new(edit_display).style(THEME.input_style(true));
            frame.render_widget(edit_input, chunks[1]);
        }

        // Hint
        let hint_text = if form_state.editing {
            "Enter: save | Esc: cancel edit"
        } else {
            "j/k: navigate | e: edit | Enter: apply | Esc: cancel"
        };
        let hint = Paragraph::new(hint_text)
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[2]);
    }
}
