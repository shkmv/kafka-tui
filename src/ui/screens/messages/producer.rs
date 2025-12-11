use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::AppState;
use crate::ui::theme::THEME;

pub struct MessageProducerScreen;

impl MessageProducerScreen {
    pub fn render(frame: &mut Frame, area: Rect, _state: &AppState, topic_name: &str) {
        let block = Block::default()
            .title(format!(" Produce Message to: {} ", topic_name))
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // TODO: Implement full message producer form
        // - Key input (optional)
        // - Value input (with JSON validation/formatting)
        // - Headers input
        // - Partition selection

        let placeholder = Paragraph::new("Message producer form - coming soon\n\nPress Esc to cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);

        frame.render_widget(placeholder, inner);
    }
}
