use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
};

use crate::app::state::AppState;
use crate::ui::layout::{messages_layout, messages_layout_collapsed};
use crate::ui::theme::THEME;

pub struct MessageBrowserScreen;

impl MessageBrowserScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState, topic_name: &str) {
        let layout = if state.messages_state.detail_expanded {
            messages_layout(area)
        } else {
            messages_layout_collapsed(area)
        };

        // Render toolbar
        Self::render_toolbar(frame, layout.toolbar, state, topic_name);

        // Render message list
        Self::render_list(frame, layout.list, state);

        // Render message detail (if expanded)
        if state.messages_state.detail_expanded && layout.detail.height > 0 {
            Self::render_detail(frame, layout.detail, state);
        }
    }

    fn render_toolbar(frame: &mut Frame, area: Rect, state: &AppState, topic_name: &str) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(THEME.border_style(false));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(30),      // Topic name
                Constraint::Length(15),   // Consuming status
                Constraint::Length(15),   // Message count
            ])
            .split(inner);

        // Topic name
        let topic_text = format!(" Messages: {}", topic_name);
        let topic_widget = Paragraph::new(topic_text).style(THEME.title_style());
        frame.render_widget(topic_widget, chunks[0]);

        // Consuming status
        let status = if state.messages_state.consumer_running {
            Span::styled(" Live", THEME.success_style())
        } else {
            Span::styled(" Paused", THEME.muted_style())
        };
        let status_widget = Paragraph::new(status);
        frame.render_widget(status_widget, chunks[1]);

        // Message count
        let count = format!("{} msgs ", state.messages_state.messages.len());
        let count_widget = Paragraph::new(count)
            .style(THEME.muted_style())
            .alignment(Alignment::Right);
        frame.render_widget(count_widget, chunks[2]);
    }

    fn render_list(frame: &mut Frame, area: Rect, state: &AppState) {
        let focused = !state.ui_state.sidebar_focused;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(THEME.border_style(focused));

        let inner = block.inner(area);

        // Loading state
        if state.messages_state.loading {
            let loading = Paragraph::new(" Loading messages...")
                .style(THEME.loading_style())
                .block(block);
            frame.render_widget(loading, area);
            return;
        }

        if state.messages_state.messages.is_empty() {
            let empty = Paragraph::new("No messages. Press 'p' to produce a message.")
                .style(THEME.muted_style())
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(empty, area);
            return;
        }

        frame.render_widget(block, area);

        // Table header
        let header = Row::new(vec![
            Cell::from(" Partition").style(THEME.table_header_style()),
            Cell::from("Offset").style(THEME.table_header_style()),
            Cell::from("Timestamp").style(THEME.table_header_style()),
            Cell::from("Key").style(THEME.table_header_style()),
            Cell::from("Value (preview)").style(THEME.table_header_style()),
        ])
        .height(1);

        // Table rows
        let rows: Vec<Row> = state
            .messages_state
            .messages
            .iter()
            .map(|msg| {
                let timestamp = msg
                    .timestamp
                    .map(|ts| ts.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| "-".to_string());

                let key = msg.key.as_deref().unwrap_or("-").to_string();
                let key_display = if key.len() > 15 {
                    format!("{}...", &key[..12])
                } else {
                    key
                };

                let value_preview = if msg.value.len() > 50 {
                    format!("{}...", &msg.value[..47])
                } else {
                    msg.value.clone()
                };
                // Replace newlines for preview
                let value_preview = value_preview.replace('\n', " ");

                Row::new(vec![
                    Cell::from(format!(" {}", msg.partition)).style(THEME.partition_style()),
                    Cell::from(msg.offset.to_string()).style(THEME.offset_style()),
                    Cell::from(timestamp),
                    Cell::from(key_display),
                    Cell::from(value_preview),
                ])
                .height(1)
            })
            .collect();

        let widths = [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Min(20),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(THEME.selected_style())
            .highlight_symbol(" ");

        let mut table_state = TableState::default();
        table_state.select(Some(state.messages_state.selected_index));

        frame.render_stateful_widget(table, inner, &mut table_state);
    }

    fn render_detail(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .title(" Message Detail ")
            .borders(Borders::ALL)
            .border_style(THEME.border_style(false));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let selected_message = state.messages_state.selected_message();

        if let Some(msg) = selected_message {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(1), // Metadata line
                    Constraint::Length(1), // Separator
                    Constraint::Min(3),    // Value
                ])
                .split(inner);

            // Metadata line
            let timestamp = msg
                .timestamp
                .map(|ts| ts.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "-".to_string());
            let key = msg.key.as_deref().unwrap_or("-");

            let metadata = Line::from(vec![
                Span::styled("Partition: ", THEME.muted_style()),
                Span::styled(msg.partition.to_string(), THEME.partition_style()),
                Span::styled("  Offset: ", THEME.muted_style()),
                Span::styled(msg.offset.to_string(), THEME.offset_style()),
                Span::styled("  Time: ", THEME.muted_style()),
                Span::styled(timestamp, THEME.normal_style()),
                Span::styled("  Key: ", THEME.muted_style()),
                Span::styled(key, THEME.normal_style()),
            ]);
            frame.render_widget(Paragraph::new(metadata), chunks[0]);

            // Value
            let value_widget = Paragraph::new(msg.value.clone())
                .style(THEME.normal_style())
                .wrap(Wrap { trim: false });
            frame.render_widget(value_widget, chunks[2]);
        } else {
            let empty = Paragraph::new("Select a message to view details")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, inner);
        }
    }
}
