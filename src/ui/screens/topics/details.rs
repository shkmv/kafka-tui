use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::state::AppState;
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

        // Find the topic in state
        let topic = state.topics_state.topics.iter().find(|t| t.name == topic_name);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(10), // Topic info table
                Constraint::Length(2),  // Spacer
                Constraint::Min(5),     // Partitions info
                Constraint::Length(2),  // Hints
            ])
            .split(inner);

        if let Some(topic) = topic {
            // Topic info table
            let info_rows = vec![
                Row::new(vec![
                    Cell::from(" Name:").style(THEME.muted_style()),
                    Cell::from(topic.name.clone()).style(THEME.normal_style()),
                ]),
                Row::new(vec![
                    Cell::from(" Partitions:").style(THEME.muted_style()),
                    Cell::from(topic.partition_count.to_string()).style(THEME.info_style()),
                ]),
                Row::new(vec![
                    Cell::from(" Replication Factor:").style(THEME.muted_style()),
                    Cell::from(topic.replication_factor.to_string()).style(THEME.info_style()),
                ]),
                Row::new(vec![
                    Cell::from(" Message Count:").style(THEME.muted_style()),
                    Cell::from(
                        topic.message_count
                            .map(|c| format_number(c))
                            .unwrap_or_else(|| "N/A".to_string())
                    ).style(THEME.normal_style()),
                ]),
                Row::new(vec![
                    Cell::from(" Internal:").style(THEME.muted_style()),
                    Cell::from(if topic.is_internal { "Yes" } else { "No" }).style(
                        if topic.is_internal { THEME.warning_style() } else { THEME.normal_style() }
                    ),
                ]),
            ];

            let info_table = Table::new(
                info_rows,
                [Constraint::Length(22), Constraint::Min(20)]
            );
            frame.render_widget(info_table, chunks[0]);

            // Partitions section
            let partitions_block = Block::default()
                .title(" Partitions ")
                .borders(Borders::ALL)
                .border_style(THEME.border_style(false));

            let partitions_inner = partitions_block.inner(chunks[2]);
            frame.render_widget(partitions_block, chunks[2]);

            // Mock partition data
            let partition_header = Row::new(vec![
                Cell::from(" Partition").style(THEME.table_header_style()),
                Cell::from("Leader").style(THEME.table_header_style()),
                Cell::from("Replicas").style(THEME.table_header_style()),
                Cell::from("ISR").style(THEME.table_header_style()),
                Cell::from("Offset").style(THEME.table_header_style()),
            ]).height(1);

            let partition_rows: Vec<Row> = (0..topic.partition_count.min(10))
                .map(|i| {
                    Row::new(vec![
                        Cell::from(format!(" {}", i)),
                        Cell::from("1"),
                        Cell::from("[1, 2, 3]"),
                        Cell::from("[1, 2, 3]"),
                        Cell::from(format!("{}", 1000 + i * 100)),
                    ])
                })
                .collect();

            let partition_table = Table::new(
                partition_rows,
                [
                    Constraint::Length(12),
                    Constraint::Length(8),
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Min(10),
                ]
            )
            .header(partition_header)
            .row_highlight_style(THEME.selected_style());

            frame.render_widget(partition_table, partitions_inner);
        } else {
            let not_found = Paragraph::new(format!("Topic '{}' not found", topic_name))
                .style(THEME.error_style())
                .alignment(Alignment::Center);
            frame.render_widget(not_found, chunks[0]);
        }

        // Hints
        let hints = Paragraph::new("Press 'm' to view messages | Esc to go back")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);
        frame.render_widget(hints, chunks[3]);
    }
}

fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
