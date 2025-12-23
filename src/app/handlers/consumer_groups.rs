//! Consumer group action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, ConsumerGroupDetailTab, Level, Screen};

use super::super::update::toast;

/// Handle consumer group actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::FetchConsumerGroups => {
            state.consumer_groups_state.loading = true;
            Some(Command::FetchConsumerGroupList)
        }

        Action::ConsumerGroupsFetched(groups) => {
            state.consumer_groups_state.groups = groups.clone();
            state.consumer_groups_state.loading = false;
            state.consumer_groups_state.selected_index = 0;
            Some(Command::None)
        }

        Action::ConsumerGroupsFetchFailed(e) => {
            state.consumer_groups_state.loading = false;
            toast(state, &format!("Failed to fetch groups: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::SelectConsumerGroup(i) => {
            if *i < state.consumer_groups_state.filtered_groups().len() {
                state.consumer_groups_state.selected_index = *i;
            }
            Some(Command::None)
        }

        Action::FilterConsumerGroups(f) => {
            state.consumer_groups_state.filter = f.clone();
            state.consumer_groups_state.selected_index = 0;
            Some(Command::None)
        }

        Action::ClearConsumerGroupFilter => {
            state.consumer_groups_state.filter.clear();
            state.consumer_groups_state.selected_index = 0;
            Some(Command::None)
        }

        Action::ViewConsumerGroupDetails(id) => {
            state.screen_history.push(state.active_screen.clone());
            state.consumer_groups_state.current_detail = None;
            state.consumer_groups_state.detail_tab = ConsumerGroupDetailTab::default();
            state.active_screen = Screen::ConsumerGroupDetails { group_id: id.clone() };
            Some(Command::FetchConsumerGroupDetails(id.clone()))
        }

        Action::ConsumerGroupDetailsFetched(detail) => {
            state.consumer_groups_state.current_detail = Some(detail.clone());
            Some(Command::None)
        }

        Action::ConsumerGroupDetailsFetchFailed(e) => {
            toast(state, e, Level::Error);
            Some(Command::None)
        }

        Action::SwitchConsumerGroupDetailTab => {
            state.consumer_groups_state.detail_tab = match state.consumer_groups_state.detail_tab {
                ConsumerGroupDetailTab::Members => ConsumerGroupDetailTab::Offsets,
                ConsumerGroupDetailTab::Offsets => ConsumerGroupDetailTab::Members,
            };
            Some(Command::None)
        }

        _ => None,
    }
}
