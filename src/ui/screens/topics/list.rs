use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::app::state::AppState;
use crate::ui::layout::topics_list_layout;
use crate::ui::theme::THEME;

pub struct TopicsListScreen;

impl TopicsListScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let layout = topics_list_layout(area);

        // Render toolbar
        Self::render_toolbar(frame, layout.toolbar, state);

        // Render topics list
        Self::render_list(frame, layout.list, state);

        // Render details panel
        Self::render_details(frame, layout.details, state);
    }

    fn render_toolbar(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(THEME.border_style(false));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(20),      // Filter info
                Constraint::Length(20),   // Topic count
            ])
            .split(inner);

        // Filter display
        let filter_text = if state.topics_state.filter.is_empty() {
            Span::styled(" Topics", THEME.title_style())
        } else {
            Span::styled(
                format!(" Filter: {}", state.topics_state.filter),
                THEME.info_style(),
            )
        };
        let filter_widget = Paragraph::new(filter_text);
        frame.render_widget(filter_widget, chunks[0]);

        // Topic count
        let filtered_count = state.topics_state.filtered_topics().len();
        let total_count = state.topics_state.topics.len();
        let count_text = if filtered_count == total_count {
            format!("{} topics ", total_count)
        } else {
            format!("{}/{} topics ", filtered_count, total_count)
        };
        let count_widget = Paragraph::new(count_text)
            .style(THEME.muted_style())
            .alignment(Alignment::Right);
        frame.render_widget(count_widget, chunks[1]);
    }

    fn render_list(frame: &mut Frame, area: Rect, state: &AppState) {
        let focused = !state.ui_state.sidebar_focused;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(THEME.border_style(focused));

        let inner = block.inner(area);

        // Loading state
        if state.topics_state.loading {
            let loading = Paragraph::new(" Loading topics...")
                .style(THEME.loading_style())
                .block(block);
            frame.render_widget(loading, area);
            return;
        }

        let filtered_topics = state.topics_state.filtered_topics();

        if filtered_topics.is_empty() {
            let empty_message = if state.topics_state.filter.is_empty() {
                "No topics found. Press 'n' to create one."
            } else {
                "No topics match the filter."
            };
            let empty = Paragraph::new(empty_message)
                .style(THEME.muted_style())
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(empty, area);
            return;
        }

        frame.render_widget(block, area);

        // Table header
        let header = Row::new(vec![
            Cell::from(" Name").style(THEME.table_header_style()),
            Cell::from("Partitions").style(THEME.table_header_style()),
            Cell::from("Replication").style(THEME.table_header_style()),
        ])
        .height(1);

        // Table rows
        let rows: Vec<Row> = filtered_topics
            .iter()
            .map(|topic| {
                let style = if topic.is_internal {
                    THEME.topic_internal_style()
                } else {
                    THEME.normal_style()
                };

                let name = if topic.is_internal {
                    format!(" {} (internal)", topic.name)
                } else {
                    format!(" {}", topic.name)
                };

                Row::new(vec![
                    Cell::from(name).style(style),
                    Cell::from(topic.partition_count.to_string()).style(THEME.partition_style()),
                    Cell::from(topic.replication_factor.to_string()),
                ])
                .height(1)
            })
            .collect();

        let widths = [
            Constraint::Min(30),
            Constraint::Length(12),
            Constraint::Length(12),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(THEME.selected_style())
            .highlight_symbol(" ");

        let mut table_state = TableState::default();
        table_state.select(Some(state.topics_state.selected_index));

        frame.render_stateful_widget(table, inner, &mut table_state);
    }

    fn render_details(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .title(" Topic Details ")
            .borders(Borders::ALL)
            .border_style(THEME.border_style(false));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let selected_topic = state.topics_state.selected_topic();

        if let Some(topic) = selected_topic {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(2), // Name
                    Constraint::Length(1), // Separator
                    Constraint::Length(1), // Partitions
                    Constraint::Length(1), // Replication
                    Constraint::Length(1), // Internal
                    Constraint::Min(1),    // Spacer
                ])
                .split(inner);

            // Topic name
            let name_line = Line::from(vec![
                Span::styled(&topic.name, THEME.header_style()),
            ]);
            frame.render_widget(Paragraph::new(name_line), chunks[0]);

            // Partitions
            let partitions_line = Line::from(vec![
                Span::styled("Partitions: ", THEME.muted_style()),
                Span::styled(
                    topic.partition_count.to_string(),
                    THEME.partition_style(),
                ),
            ]);
            frame.render_widget(Paragraph::new(partitions_line), chunks[2]);

            // Replication
            let replication_line = Line::from(vec![
                Span::styled("Replication: ", THEME.muted_style()),
                Span::styled(
                    topic.replication_factor.to_string(),
                    THEME.normal_style(),
                ),
            ]);
            frame.render_widget(Paragraph::new(replication_line), chunks[3]);

            // Internal flag
            let internal_line = Line::from(vec![
                Span::styled("Internal: ", THEME.muted_style()),
                Span::styled(
                    if topic.is_internal { "Yes" } else { "No" },
                    if topic.is_internal {
                        THEME.warning_style()
                    } else {
                        THEME.normal_style()
                    },
                ),
            ]);
            frame.render_widget(Paragraph::new(internal_line), chunks[4]);
        } else {
            let empty = Paragraph::new("Select a topic to view details")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(empty, inner);
        }
    }
}
