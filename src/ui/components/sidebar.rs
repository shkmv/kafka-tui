use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use strum::IntoEnumIterator;

use crate::app::state::{AppState, SidebarItem};
use crate::ui::theme::THEME;

pub struct Sidebar;

impl Sidebar {
    pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
        let focused = state.ui_state.sidebar_focused;

        let block = Block::default()
            .title(" Navigation ")
            .borders(Borders::ALL)
            .border_style(THEME.border_style(focused));

        let items: Vec<ListItem> = SidebarItem::iter()
            .map(|item| {
                let icon = match item {
                    SidebarItem::Topics => "",
                    SidebarItem::ConsumerGroups => "󰡨",
                    SidebarItem::Brokers => "",
                };
                let label = match item {
                    SidebarItem::Topics => "Topics",
                    SidebarItem::ConsumerGroups => "Consumers",
                    SidebarItem::Brokers => "Brokers",
                };

                let is_selected = state.ui_state.selected_sidebar_item == item;
                let style = THEME.sidebar_item_style(is_selected, focused);

                ListItem::new(format!(" {} {}", icon, label)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(THEME.highlight_style());

        // Calculate which item is selected
        let selected_index = SidebarItem::iter()
            .position(|item| item == state.ui_state.selected_sidebar_item)
            .unwrap_or(0);

        let mut list_state = ListState::default();
        list_state.select(Some(selected_index));

        frame.render_stateful_widget(list, area, &mut list_state);
    }
}
