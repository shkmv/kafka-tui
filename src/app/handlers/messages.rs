//! Message-related action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, Level, OffsetMode, Screen};

use super::super::update::toast;

/// Handle message actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::FetchMessages {
            topic,
            offset_mode,
            partition,
        } => {
            state.messages_state.loading = true;
            state.messages_state.offset_mode = offset_mode.clone();
            state.messages_state.partition_filter = *partition;
            Some(Command::FetchMessages {
                topic: topic.clone(),
                offset_mode: offset_mode.clone(),
                partition: *partition,
                limit: 100,
            })
        }

        Action::MessagesFetched(msgs) => {
            state.messages_state.messages = msgs.clone();
            state.messages_state.loading = false;
            state.messages_state.selected_index = 0;
            Some(Command::None)
        }

        Action::MessageReceived(msg) => {
            state.messages_state.messages.push(msg.clone());
            Some(Command::None)
        }

        Action::MessagesFetchFailed(e) => {
            state.messages_state.loading = false;
            toast(
                state,
                &format!("Failed to fetch messages: {}", e),
                Level::Error,
            );
            Some(Command::None)
        }

        Action::SelectMessage(i) => {
            if *i < state.messages_state.messages.len() {
                state.messages_state.selected_index = *i;
            }
            Some(Command::None)
        }

        Action::SetOffsetMode(m) => {
            state.messages_state.offset_mode = m.clone();
            Some(Command::None)
        }

        Action::SetPartitionFilter(p) => {
            state.messages_state.partition_filter = *p;
            Some(Command::None)
        }

        Action::StartConsuming { topic } => {
            state.messages_state.consumer_running = true;
            Some(Command::StartMessageConsumer {
                topic: topic.clone(),
                offset_mode: state.messages_state.offset_mode.clone(),
                partition: state.messages_state.partition_filter,
            })
        }

        Action::StopConsuming => {
            state.messages_state.consumer_running = false;
            Some(Command::StopMessageConsumer)
        }

        Action::ProduceMessage {
            topic,
            key,
            value,
            headers,
        } => Some(Command::ProduceKafkaMessage {
            topic: topic.clone(),
            key: key.clone(),
            value: value.clone(),
            headers: headers.clone(),
        }),

        Action::MessageProduced => {
            state.ui_state.active_modal = None;
            toast(state, "Message produced", Level::Success);
            if let Screen::Messages { topic_name } = &state.active_screen {
                Some(Command::FetchMessages {
                    topic: topic_name.clone(),
                    offset_mode: OffsetMode::Latest,
                    partition: None,
                    limit: 100,
                })
            } else {
                Some(Command::None)
            }
        }

        Action::MessageProduceFailed(e) => {
            toast(state, &format!("Produce failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::ToggleMessageDetail => {
            state.messages_state.detail_expanded = !state.messages_state.detail_expanded;
            Some(Command::None)
        }

        Action::ClearMessages => {
            state.messages_state.messages.clear();
            state.messages_state.selected_index = 0;
            Some(Command::None)
        }

        _ => None,
    }
}
