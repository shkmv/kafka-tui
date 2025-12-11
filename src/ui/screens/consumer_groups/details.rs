use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
};

use crate::app::state::{AppState, ConsumerGroupDetail, ConsumerGroupDetailTab};
use crate::ui::theme::THEME;

pub struct ConsumerGroupDetailsScreen;

impl ConsumerGroupDetailsScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState, group_id: &str) {
        let block = Block::default()
            .title(format!(" Consumer Group: {} ", group_id))
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Tabs
                Constraint::Min(10),    // Content
                Constraint::Length(1),  // Hints
            ])
            .split(inner);

        // Tabs
        let tabs = Tabs::new(vec!["Members", "Offsets"])
            .select(match state.consumer_groups_state.detail_tab {
                ConsumerGroupDetailTab::Members => 0,
                ConsumerGroupDetailTab::Offsets => 1,
            })
            .style(THEME.muted_style())
            .highlight_style(THEME.header_style())
            .divider(" | ");
        frame.render_widget(tabs, chunks[0]);

        // Content based on tab
        match &state.consumer_groups_state.current_detail {
            Some(detail) => {
                match state.consumer_groups_state.detail_tab {
                    ConsumerGroupDetailTab::Members => Self::render_members(frame, chunks[1], detail),
                    ConsumerGroupDetailTab::Offsets => Self::render_offsets(frame, chunks[1], detail),
                }
            }
            None => {
                let loading = Paragraph::new("Loading...")
                    .style(THEME.loading_style())
                    .alignment(Alignment::Center);
                frame.render_widget(loading, chunks[1]);
            }
        }

        // Hints
        let hints = Paragraph::new(" [Tab/h/l] Switch tab | [F5] Refresh | [Esc] Back")
            .style(THEME.muted_style());
        frame.render_widget(hints, chunks[2]);
    }

    fn render_members(frame: &mut Frame, area: Rect, detail: &ConsumerGroupDetail) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(5)])
            .split(area);

        // Group state info
        let state_style = THEME.consumer_group_state_style(&detail.state);
        let info = Paragraph::new(format!(
            " State: {}  |  Members: {}",
            detail.state, detail.members.len()
        )).style(state_style);
        frame.render_widget(info, chunks[0]);

        if detail.members.is_empty() {
            let empty = Paragraph::new("No active members")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, chunks[1]);
            return;
        }

        let header = Row::new(vec![
            Cell::from(" Client ID").style(THEME.table_header_style()),
            Cell::from("Host").style(THEME.table_header_style()),
            Cell::from("Assignments").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = detail.members.iter().map(|m| {
            let assignments = if m.assignments.is_empty() {
                "None".to_string()
            } else {
                m.assignments.iter()
                    .map(|a| format!("{}:{}", a.topic, a.partition))
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            Row::new(vec![
                Cell::from(format!(" {}", m.client_id)),
                Cell::from(m.client_host.clone()),
                Cell::from(assignments),
            ])
        }).collect();

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(25),
                Constraint::Percentage(45),
            ]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, chunks[1]);
    }

    fn render_offsets(frame: &mut Frame, area: Rect, detail: &ConsumerGroupDetail) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(5)])
            .split(area);

        // Summary
        let total_lag: i64 = detail.offsets.iter().map(|o| o.lag).sum();
        let partition_count = detail.offsets.len();

        let lag_style = THEME.lag_style(total_lag);
        let info = Paragraph::new(format!(
            " Partitions: {}  |  Total Lag: {}",
            partition_count, format_number(total_lag)
        )).style(lag_style);
        frame.render_widget(info, chunks[0]);

        if detail.offsets.is_empty() {
            let empty = Paragraph::new("No committed offsets")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, chunks[1]);
            return;
        }

        let header = Row::new(vec![
            Cell::from(" Topic").style(THEME.table_header_style()),
            Cell::from("Partition").style(THEME.table_header_style()),
            Cell::from("Current").style(THEME.table_header_style()),
            Cell::from("End").style(THEME.table_header_style()),
            Cell::from("Lag").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = detail.offsets.iter().map(|o| {
            Row::new(vec![
                Cell::from(format!(" {}", o.topic)),
                Cell::from(o.partition.to_string()).style(THEME.partition_style()),
                Cell::from(format_number(o.current_offset)).style(THEME.offset_style()),
                Cell::from(format_number(o.log_end_offset)).style(THEME.offset_style()),
                Cell::from(format_number(o.lag)).style(THEME.lag_style(o.lag)),
            ])
        }).collect();

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(35),
                Constraint::Length(10),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Min(10),
            ]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, chunks[1]);
    }
}

fn format_number(n: i64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
