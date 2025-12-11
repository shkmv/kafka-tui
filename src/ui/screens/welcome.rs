use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::state::AppState;
use crate::ui::theme::THEME;

pub struct WelcomeScreen;

impl WelcomeScreen {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = Block::default()
            .title(" Welcome to Kafka TUI ")
            .title_style(THEME.header_style())
            .borders(Borders::ALL)
            .border_style(THEME.border_style(true));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Layout: logo/title, connection list, hint
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(6),  // Logo/title
                Constraint::Min(10),    // Connection list
                Constraint::Length(3),  // Hints
            ])
            .split(inner);

        // ASCII Logo
        let logo = r#"
  _  __      __ _           _____ _   _ ___
 | |/ /__ _ / _| | ____ _  |_   _| | | |_ _|
 | ' // _` | |_| |/ / _` |   | | | | | || |
 | . \ (_| |  _|   < (_| |   | | | |_| || |
 |_|\_\__,_|_| |_|\_\__,_|   |_|  \___/|___|
        "#;

        let logo_widget = Paragraph::new(logo)
            .style(Style::default().fg(THEME.accent_secondary))
            .alignment(Alignment::Center);
        frame.render_widget(logo_widget, chunks[0]);

        // Connection profiles list
        if state.connection.available_profiles.is_empty() {
            let no_profiles = Paragraph::new("No saved connections.\nPress 'n' to create a new connection.")
                .style(THEME.muted_style())
                .alignment(Alignment::Center);
            frame.render_widget(no_profiles, chunks[1]);
        } else {
            let items: Vec<ListItem> = state
                .connection
                .available_profiles
                .iter()
                .map(|profile| {
                    let text = format!(
                        " {}  {} ({})",
                        "", // Server icon
                        profile.name,
                        profile.brokers
                    );
                    ListItem::new(text).style(THEME.normal_style())
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(" Saved Connections ")
                        .borders(Borders::ALL)
                        .border_style(THEME.border_style(true)),
                )
                .highlight_style(THEME.selected_style())
                .highlight_symbol(" ");

            let mut list_state = ListState::default();
            list_state.select(Some(state.connection.selected_index));

            frame.render_stateful_widget(list, chunks[1], &mut list_state);
        }

        // Hints
        let hints = Line::from(vec![
            Span::styled("[Enter]", THEME.key_hint_style()),
            Span::styled(" Connect  ", THEME.muted_style()),
            Span::styled("[n]", THEME.key_hint_style()),
            Span::styled(" New connection  ", THEME.muted_style()),
            Span::styled("[q]", THEME.key_hint_style()),
            Span::styled(" Quit", THEME.muted_style()),
        ]);

        let hints_widget = Paragraph::new(hints)
            .alignment(Alignment::Center);
        frame.render_widget(hints_widget, chunks[2]);
    }
}
