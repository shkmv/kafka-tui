use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::state::AppState;
use crate::ui::theme::THEME;

pub struct BrokersScreen;

impl BrokersScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .title(" Brokers ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(!state.ui_state.sidebar_focused));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if state.brokers_state.loading {
            let loading = Paragraph::new("Loading brokers...")
                .style(THEME.loading_style())
                .alignment(Alignment::Center);
            frame.render_widget(loading, inner);
            return;
        }

        if state.brokers_state.brokers.is_empty() {
            let empty = Paragraph::new("No brokers found")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, inner);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(5)])
            .split(inner);

        // Summary
        let broker_count = state.brokers_state.brokers.len();
        let controller = state.brokers_state.brokers.iter()
            .find(|b| b.is_controller)
            .map(|b| format!("Controller: {}", b.id))
            .unwrap_or_else(|| "Controller: Unknown".to_string());

        let summary = Paragraph::new(format!(
            " {} brokers | {}",
            broker_count, controller
        )).style(THEME.muted_style());
        frame.render_widget(summary, chunks[0]);

        // Table
        let header = Row::new(vec![
            Cell::from(" ID").style(THEME.table_header_style()),
            Cell::from("Host").style(THEME.table_header_style()),
            Cell::from("Port").style(THEME.table_header_style()),
            Cell::from("Role").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = state.brokers_state.brokers.iter().map(|b| {
            let role = if b.is_controller { "Controller" } else { "Follower" };
            let role_style = if b.is_controller { THEME.success_style() } else { THEME.normal_style() };

            Row::new(vec![
                Cell::from(format!(" {}", b.id)).style(THEME.partition_style()),
                Cell::from(b.host.clone()),
                Cell::from(b.port.to_string()),
                Cell::from(role).style(role_style),
            ])
        }).collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(8),
                Constraint::Percentage(50),
                Constraint::Length(10),
                Constraint::Min(15),
            ]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, chunks[1]);
    }
}
