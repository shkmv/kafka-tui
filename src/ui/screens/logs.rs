use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::state::{AppState, Level};
use crate::ui::theme::THEME;
use crate::ui::widgets::render_empty;

pub struct LogsScreen;

impl LogsScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .title(" Logs ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(!state.ui_state.sidebar_focused));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(5)])
            .split(inner);

        Self::render_toolbar(frame, chunks[0], state);
        Self::render_entries(frame, chunks[1], state);
    }

    fn render_toolbar(frame: &mut Frame, area: Rect, state: &AppState) {
        let filter_text = match state.logs_state.filter_level {
            None => "All",
            Some(Level::Error) => "Errors",
            Some(Level::Warning) => "Warnings",
            Some(Level::Success) => "Success",
            Some(Level::Info) => "Info",
        };

        let total = state.logs_state.entries.len();
        let filtered = state.logs_state.filtered_entries().len();

        let toolbar = Paragraph::new(format!(
            " [c] Clear  [f] Filter: {}  |  {} / {} entries",
            filter_text, filtered, total
        )).style(THEME.muted_style());
        frame.render_widget(toolbar, area);
    }

    fn render_entries(frame: &mut Frame, area: Rect, state: &AppState) {
        let entries = state.logs_state.filtered_entries();

        if entries.is_empty() {
            render_empty(frame, area, "No log entries");
            return;
        }

        let header = Row::new(vec![
            Cell::from(" Time").style(THEME.table_header_style()),
            Cell::from("Level").style(THEME.table_header_style()),
            Cell::from("Message").style(THEME.table_header_style()),
        ]).height(1);

        let rows: Vec<Row> = entries.iter().enumerate().map(|(i, entry)| {
            let time = entry.timestamp.format("%H:%M:%S").to_string();
            let level_text = format!("{} {}", entry.level.icon(), entry.level.label());

            let row_style = if i == state.logs_state.selected_index {
                THEME.selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!(" {}", time)).style(THEME.muted_style()),
                Cell::from(level_text).style(entry.level.style()),
                Cell::from(entry.message.clone()),
            ]).style(row_style)
        }).collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(10),
                Constraint::Length(8),
                Constraint::Min(30),
            ]
        )
        .header(header)
        .row_highlight_style(THEME.selected_style());

        frame.render_widget(table, area);
    }
}
