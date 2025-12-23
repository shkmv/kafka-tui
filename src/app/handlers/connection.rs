//! Connection-related action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, ConfirmAction, ConnectionStatus, Level, ModalType, Screen};

use super::super::update::toast;

/// Handle connection actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::Connect(profile) => {
            state.connection.status = ConnectionStatus::Connecting;
            state.connection.active_profile = Some(profile.clone());
            toast(state, &format!("Connecting to {}...", profile.brokers), Level::Info);
            Some(Command::ConnectToKafka(profile.clone()))
        }

        Action::ConnectionSuccess => {
            state.connection.status = ConnectionStatus::Connected;
            state.active_screen = Screen::Topics;
            toast(state, "Connected", Level::Success);
            let mut cmds = vec![Command::FetchTopicList, Command::FetchConsumerGroupList];
            if let Some(p) = &state.connection.active_profile {
                cmds.push(Command::SaveConnectionProfile(p.clone()));
            }
            Some(Command::Batch(cmds))
        }

        Action::ConnectionFailed(e) => {
            state.connection.status = ConnectionStatus::Error(e.clone());
            state.connection.active_profile = None;
            toast(state, &format!("Connection failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::Disconnect => {
            state.connection = Default::default();
            state.topics_state = Default::default();
            state.messages_state = Default::default();
            state.consumer_groups_state = Default::default();
            state.active_screen = Screen::Welcome;
            state.screen_history.clear();
            Some(Command::DisconnectFromKafka)
        }

        Action::LoadSavedConnections => Some(Command::LoadConnectionProfiles),

        Action::ConnectionsLoaded(p) => {
            state.connection.available_profiles = p.clone();
            Some(Command::None)
        }

        Action::SaveConnection(p) => Some(Command::SaveConnectionProfile(p.clone())),

        Action::RequestDeleteConnection => {
            if let Some(profile) = state
                .connection
                .available_profiles
                .get(state.connection.selected_index)
                .cloned()
            {
                state.ui_state.active_modal = Some(ModalType::Confirm {
                    title: "Delete Connection".into(),
                    message: format!("Delete '{}'?", profile.name),
                    action: ConfirmAction::DeleteConnection(profile.id),
                });
            }
            Some(Command::None)
        }

        Action::DeleteConnection(id) => Some(Command::DeleteConnectionProfile(*id)),

        Action::ConnectionDeleted(id) => {
            state.connection.available_profiles.retain(|p| p.id != *id);
            state.connection.selected_index = state
                .connection
                .selected_index
                .min(state.connection.available_profiles.len().saturating_sub(1));
            toast(state, "Connection deleted", Level::Success);
            Some(Command::None)
        }

        _ => None,
    }
}
