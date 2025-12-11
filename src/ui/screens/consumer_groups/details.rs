use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::AppState;
use crate::ui::theme::THEME;

pub struct ConsumerGroupDetailsScreen;

impl ConsumerGroupDetailsScreen {
    pub fn render(frame: &mut Frame, area: Rect, _state: &AppState, group_id: &str) {
        let block = Block::default()
            .title(format!(" Consumer Group: {} ", group_id))
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // TODO: Implement full consumer group details view
        // - Group info (state, coordinator, protocol)
        // - Members table
        // - Partition offsets table with lag

        let placeholder = Paragraph::new("Consumer group details view - coming soon\n\nPress Esc to go back")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);

        frame.render_widget(placeholder, inner);
    }
}
