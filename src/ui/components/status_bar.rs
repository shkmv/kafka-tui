use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

use crate::app::state::AppState;
use crate::events::key_bindings::get_help_text;
use crate::ui::theme::THEME;

pub struct StatusBar;

impl StatusBar {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(40),      // Key hints
                Constraint::Length(30),   // Connection info
            ])
            .split(area);

        // Key hints
        let help_items = get_help_text(&state.active_screen);
        let hints: Vec<Span> = help_items
            .iter()
            .take(6)
            .flat_map(|(key, desc)| {
                vec![
                    Span::styled(format!(" [{}]", key), THEME.key_hint_style()),
                    Span::styled(format!(" {} ", desc), THEME.key_desc_style()),
                ]
            })
            .collect();

        let hints_line = Line::from(hints);
        let hints_paragraph = Paragraph::new(hints_line);
        frame.render_widget(hints_paragraph, chunks[0]);

        // Connection info
        let conn_info = if let Some(ref profile) = state.connection.active_profile {
            profile.brokers.clone()
        } else {
            "Not connected".to_string()
        };

        let conn_style = if state.connection.active_profile.is_some() {
            THEME.muted_style()
        } else {
            THEME.status_disconnected()
        };

        let conn_paragraph = Paragraph::new(conn_info)
            .style(conn_style)
            .alignment(Alignment::Right);
        frame.render_widget(conn_paragraph, chunks[1]);
    }

    pub fn render_loading(frame: &mut Frame, area: Rect, message: &str) {
        let loading = Paragraph::new(format!(" {} ", message))
            .style(THEME.loading_style());
        frame.render_widget(loading, area);
    }
}
