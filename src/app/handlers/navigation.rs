//! Navigation action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, Navigable, Screen};

/// Handle navigation actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::Navigate(screen) => {
            if &state.active_screen == screen {
                return Some(Command::None);
            }
            state.screen_history.push(state.active_screen.clone());
            state.active_screen = screen.clone();
            Some(match screen {
                Screen::Topics => Command::FetchTopicList,
                Screen::ConsumerGroups => Command::FetchConsumerGroupList,
                Screen::Brokers => Command::FetchBrokerList,
                Screen::Messages { topic_name } => Command::FetchMessages {
                    topic: topic_name.clone(),
                    offset_mode: state.messages_state.offset_mode.clone(),
                    partition: state.messages_state.partition_filter,
                    limit: 100,
                },
                _ => Command::None,
            })
        }

        Action::GoBack => {
            if state.ui_state.active_modal.take().is_some() {
                return Some(Command::None);
            }
            if std::mem::take(&mut state.ui_state.show_help) {
                return Some(Command::None);
            }
            if let Some(prev) = state.screen_history.pop() {
                state.active_screen = prev;
            }
            Some(Command::None)
        }

        Action::FocusSidebar => {
            state.ui_state.sidebar_focused = true;
            Some(Command::None)
        }
        Action::FocusContent => {
            state.ui_state.sidebar_focused = false;
            Some(Command::None)
        }

        Action::SelectSidebarItem(item) => {
            state.ui_state.selected_sidebar_item = item.clone();
            handle(state, &Action::Navigate(item.to_screen()))
        }

        Action::MoveUp => {
            nav_up(state);
            Some(Command::None)
        }
        Action::MoveDown => {
            nav_down(state);
            Some(Command::None)
        }
        Action::MoveLeft => {
            if state.ui_state.sidebar_focused {
                sidebar_prev(state);
            }
            Some(Command::None)
        }
        Action::MoveRight => {
            if state.ui_state.sidebar_focused {
                sidebar_next(state);
            }
            Some(Command::None)
        }
        Action::PageUp => {
            for _ in 0..10 {
                nav_up(state);
            }
            Some(Command::None)
        }
        Action::PageDown => {
            for _ in 0..10 {
                nav_down(state);
            }
            Some(Command::None)
        }
        Action::ScrollToTop => {
            nav_to(state, 0);
            Some(Command::None)
        }
        Action::ScrollToBottom => {
            nav_to(state, usize::MAX);
            Some(Command::None)
        }
        Action::Select => Some(handle_select(state)),
        Action::Cancel => handle(state, &Action::GoBack),

        _ => None,
    }
}

fn nav_up(state: &mut AppState) {
    if state.ui_state.sidebar_focused {
        return sidebar_prev(state);
    }
    match &state.active_screen {
        Screen::Topics => state.topics_state.nav_up(),
        Screen::Messages { .. } => state.messages_state.nav_up(),
        Screen::ConsumerGroups => state.consumer_groups_state.nav_up(),
        Screen::Welcome => state.connection.nav_up(),
        Screen::Logs => state.logs_state.nav_up(),
        _ => {}
    }
}

fn nav_down(state: &mut AppState) {
    if state.ui_state.sidebar_focused {
        return sidebar_next(state);
    }
    match &state.active_screen {
        Screen::Topics => state.topics_state.nav_down(),
        Screen::Messages { .. } => state.messages_state.nav_down(),
        Screen::ConsumerGroups => state.consumer_groups_state.nav_down(),
        Screen::Welcome => state.connection.nav_down(),
        Screen::Logs => state.logs_state.nav_down(),
        _ => {}
    }
}

fn nav_to(state: &mut AppState, target: usize) {
    match &state.active_screen {
        Screen::Topics => state.topics_state.nav_to(target),
        Screen::Messages { .. } => state.messages_state.nav_to(target),
        Screen::ConsumerGroups => state.consumer_groups_state.nav_to(target),
        Screen::Logs => state.logs_state.nav_to(target),
        _ => {}
    }
}

fn sidebar_prev(state: &mut AppState) {
    state.ui_state.selected_sidebar_item = state.ui_state.selected_sidebar_item.prev();
}

fn sidebar_next(state: &mut AppState) {
    state.ui_state.selected_sidebar_item = state.ui_state.selected_sidebar_item.next();
}

fn handle_select(state: &mut AppState) -> Command {
    if state.ui_state.sidebar_focused {
        let item = state.ui_state.selected_sidebar_item.clone();
        state.ui_state.sidebar_focused = false;
        return handle(state, &Action::Navigate(item.to_screen())).unwrap_or(Command::None);
    }

    match &state.active_screen {
        Screen::Topics => {
            let name = state.topics_state.selected_topic().map(|t| t.name.clone());
            name.map(|n| {
                state.screen_history.push(state.active_screen.clone());
                state.messages_state.current_topic = Some(n.clone());
                state.messages_state.messages.clear();
                state.messages_state.selected_index = 0;
                state.active_screen = Screen::Messages { topic_name: n.clone() };
                Command::FetchMessages {
                    topic: n,
                    offset_mode: state.messages_state.offset_mode.clone(),
                    partition: state.messages_state.partition_filter,
                    limit: 100,
                }
            })
            .unwrap_or(Command::None)
        }
        Screen::Messages { .. } => {
            state.messages_state.detail_expanded = !state.messages_state.detail_expanded;
            Command::None
        }
        Screen::ConsumerGroups => {
            let id = state
                .consumer_groups_state
                .selected_group()
                .map(|g| g.group_id.clone());
            id.map(|i| {
                state.screen_history.push(state.active_screen.clone());
                state.consumer_groups_state.current_detail = None;
                state.consumer_groups_state.detail_tab = Default::default();
                state.active_screen = Screen::ConsumerGroupDetails { group_id: i.clone() };
                Command::FetchConsumerGroupDetails(i)
            })
            .unwrap_or(Command::None)
        }
        Screen::Welcome => {
            let profile = state
                .connection
                .available_profiles
                .get(state.connection.selected_index)
                .cloned();
            profile
                .map(|p| {
                    state.connection.status = crate::app::state::ConnectionStatus::Connecting;
                    state.connection.active_profile = Some(p.clone());
                    Command::ConnectToKafka(p)
                })
                .unwrap_or(Command::None)
        }
        _ => Command::None,
    }
}
