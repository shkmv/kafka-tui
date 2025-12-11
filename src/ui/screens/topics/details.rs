use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
};

use crate::app::state::{AppState, TopicDetailTab};
use crate::ui::theme::THEME;

pub struct TopicDetailsScreen;

impl TopicDetailsScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState, topic_name: &str) {
        let block = Block::default()
            .title(format!(" Topic: {} ", topic_name))
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
        let tabs = Tabs::new(vec!["Partitions", "Configuration"])
            .select(match state.topics_state.detail_tab {
                TopicDetailTab::Partitions => 0,
                TopicDetailTab::Config => 1,
            })
            .style(THEME.muted_style())
            .highlight_style(THEME.header_style())
            .divider(" | ");
        frame.render_widget(tabs, chunks[0]);

        // Content based on tab
        match &state.topics_state.current_detail {
            Some(detail) => {
                match state.topics_state.detail_tab {
                    TopicDetailTab::Partitions => Self::render_partitions(frame, chunks[1], detail),
                    TopicDetailTab::Config => Self::render_config(frame, chunks[1], detail),
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
        let hints = Paragraph::new(" [Tab/h/l] Switch tab | [m] Messages | [d] Delete | [Esc] Back")
            .style(THEME.muted_style());
        frame.render_widget(hints, chunks[2]);
    }

    fn render_partitions(frame: &mut Frame, area: Rect, detail: &crate::app::state::TopicDetail) {
        let header = Row::new(vec![
            Cell::from(" ID").style(THEME.table_header_style()),
            Cell::from("Leader").style(THEME.table_header_style()),
            Cell::from("Replicas").style(THEME.table_header_style()),
            Cell::from("ISR").style(THEME.table_header_style()),
            Cell::from("Low").style(THEME.table_header_style()),
            Cell::from("High").style(THEME.table_header_style()),
            Cell::from("Messages").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = detail.partitions.iter().map(|p| {
            let replicas = p.replicas.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(",");
            let isr = p.isr.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(",");
            let msg_count = p.message_count();

            Row::new(vec![
                Cell::from(format!(" {}", p.id)).style(THEME.partition_style()),
                Cell::from(p.leader.to_string()),
                Cell::from(format!("[{}]", replicas)),
                Cell::from(format!("[{}]", isr)).style(
                    if p.isr.len() < p.replicas.len() { THEME.warning_style() } else { THEME.normal_style() }
                ),
                Cell::from(format_number(p.low_watermark)).style(THEME.offset_style()),
                Cell::from(format_number(p.high_watermark)).style(THEME.offset_style()),
                Cell::from(format_number(msg_count)).style(THEME.info_style()),
            ])
        }).collect();

        // Summary
        let total_messages: i64 = detail.partitions.iter().map(|p| p.message_count()).sum();
        let partition_count = detail.partitions.len();

        let summary = format!(
            " {} partitions | {} total messages",
            partition_count,
            format_number(total_messages)
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(5)])
            .split(area);

        let summary_text = Paragraph::new(summary).style(THEME.muted_style());
        frame.render_widget(summary_text, chunks[0]);

        let table = Table::new(
            rows,
            [
                Constraint::Length(5),   // ID
                Constraint::Length(7),   // Leader
                Constraint::Length(12),  // Replicas
                Constraint::Length(12),  // ISR
                Constraint::Length(10),  // Low
                Constraint::Length(10),  // High
                Constraint::Min(10),     // Messages
            ]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, chunks[1]);
    }

    fn render_config(frame: &mut Frame, area: Rect, detail: &crate::app::state::TopicDetail) {
        if detail.config.is_empty() {
            let empty = Paragraph::new("No configuration available")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, area);
            return;
        }

        let header = Row::new(vec![
            Cell::from(" Name").style(THEME.table_header_style()),
            Cell::from("Value").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = detail.config.iter().map(|(name, value)| {
            let value_style = if value == "true" {
                THEME.success_style()
            } else if value == "false" {
                THEME.muted_style()
            } else if value.parse::<i64>().is_ok() {
                THEME.info_style()
            } else {
                THEME.normal_style()
            };

            Row::new(vec![
                Cell::from(format!(" {}", name)).style(THEME.normal_style()),
                Cell::from(value.clone()).style(value_style),
            ])
        }).collect();

        let table = Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, area);
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
