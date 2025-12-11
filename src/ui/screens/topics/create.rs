use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::AppState;
use crate::ui::theme::THEME;

pub struct TopicCreateScreen;

impl TopicCreateScreen {
    pub fn render(frame: &mut Frame, area: Rect, _state: &AppState) {
        let block = Block::default()
            .title(" Create Topic ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // TODO: Implement full topic creation form
        // - Topic name input
        // - Partitions input
        // - Replication factor input
        // - Additional config options

        let placeholder = Paragraph::new("Topic creation form - coming soon\n\nPress Esc to cancel")
            .style(THEME.muted_style())
            .alignment(Alignment::Center);

        frame.render_widget(placeholder, inner);
    }
}
