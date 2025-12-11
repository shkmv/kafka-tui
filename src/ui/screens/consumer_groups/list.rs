use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::app::state::AppState;
use crate::ui::layout::consumer_groups_layout;
use crate::ui::theme::THEME;

pub struct ConsumerGroupsListScreen;

impl ConsumerGroupsListScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let layout = consumer_groups_layout(area);

        // Render toolbar
        Self::render_toolbar(frame, layout.toolbar, state);

        // Render groups list
        Self::render_list(frame, layout.list, state);
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
                Constraint::Length(20),   // Group count
            ])
            .split(inner);

        // Filter display
        let filter_text = if state.consumer_groups_state.filter.is_empty() {
            Span::styled(" Consumer Groups", THEME.title_style())
        } else {
            Span::styled(
                format!(" Filter: {}", state.consumer_groups_state.filter),
                THEME.info_style(),
            )
        };
        let filter_widget = Paragraph::new(filter_text);
        frame.render_widget(filter_widget, chunks[0]);

        // Group count
        let filtered_count = state.consumer_groups_state.filtered_groups().len();
        let total_count = state.consumer_groups_state.groups.len();
        let count_text = if filtered_count == total_count {
            format!("{} groups ", total_count)
        } else {
            format!("{}/{} groups ", filtered_count, total_count)
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
        if state.consumer_groups_state.loading {
            let loading = Paragraph::new(" Loading consumer groups...")
                .style(THEME.loading_style())
                .block(block);
            frame.render_widget(loading, area);
            return;
        }

        let filtered_groups = state.consumer_groups_state.filtered_groups();

        if filtered_groups.is_empty() {
            let empty_message = if state.consumer_groups_state.filter.is_empty() {
                "No consumer groups found."
            } else {
                "No consumer groups match the filter."
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
            Cell::from(" Group ID").style(THEME.table_header_style()),
            Cell::from("State").style(THEME.table_header_style()),
            Cell::from("Members").style(THEME.table_header_style()),
            Cell::from("Lag").style(THEME.table_header_style()),
        ])
        .height(1);

        // Table rows
        let rows: Vec<Row> = filtered_groups
            .iter()
            .map(|group| {
                let state_style = THEME.consumer_group_state_style(&group.state);
                let lag_style = THEME.lag_style(group.total_lag);

                Row::new(vec![
                    Cell::from(format!(" {}", group.group_id)),
                    Cell::from(group.state.clone()).style(state_style),
                    Cell::from(group.members_count.to_string()),
                    Cell::from(group.total_lag.to_string()).style(lag_style),
                ])
                .height(1)
            })
            .collect();

        let widths = [
            Constraint::Min(30),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(15),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(THEME.selected_style())
            .highlight_symbol(" ");

        let mut table_state = TableState::default();
        table_state.select(Some(state.consumer_groups_state.selected_index));

        frame.render_stateful_widget(table, inner, &mut table_state);
    }
}
