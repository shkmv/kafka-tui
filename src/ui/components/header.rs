use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::{AppState, ConnectionStatus};
use crate::ui::theme::THEME;

pub struct Header;

impl Header {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(THEME.border_style(false));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Layout: title | cluster name | connection status
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(15),   // Title
                Constraint::Min(20),      // Cluster name
                Constraint::Length(25),   // Connection status
            ])
            .split(inner);

        // Title
        let title = Paragraph::new("  Kafka TUI")
            .style(THEME.header_style());
        frame.render_widget(title, chunks[0]);

        // Cluster name
        let cluster_name = if let Some(ref profile) = state.connection.active_profile {
            format!("  {}", profile.name)
        } else {
            String::new()
        };
        let cluster = Paragraph::new(cluster_name)
            .style(THEME.normal_style());
        frame.render_widget(cluster, chunks[1]);

        // Connection status
        let (status_text, status_style) = match &state.connection.status {
            ConnectionStatus::Connected => ("Connected", THEME.status_connected()),
            ConnectionStatus::Connecting => ("Connecting...", THEME.status_connecting()),
            ConnectionStatus::Disconnected => ("Disconnected", THEME.status_disconnected()),
            ConnectionStatus::Error(e) => {
                let msg = if e.len() > 15 {
                    format!("Error: {}...", &e[..12])
                } else {
                    format!("Error: {}", e)
                };
                // Return tuple with owned string - need different approach
                (Box::leak(msg.into_boxed_str()) as &str, THEME.error_style())
            }
        };

        let status = Paragraph::new(status_text)
            .style(status_style)
            .alignment(Alignment::Right);
        frame.render_widget(status, chunks[2]);
    }
}
