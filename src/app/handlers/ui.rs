//! UI and modal action handlers.

use chrono::Utc;
use uuid::Uuid;

use crate::app::actions::{Action, Command};
use crate::app::state::{
    AppState, AuthConfig, AuthType, ConfirmAction, ConnectionProfile, ConnectionStatus,
    InputAction, Level, ModalType, ToastMessage,
};
use crate::app::validation::{
    parse_new_partition_count, parse_offset, parse_partitions, parse_replication_factor,
};

/// Handle UI/modal actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::ShowHelp => {
            state.ui_state.show_help = true;
            Some(Command::None)
        }

        Action::HideHelp => {
            state.ui_state.show_help = false;
            Some(Command::None)
        }

        Action::ShowModal(m) => {
            state.ui_state.active_modal = Some(m.clone());
            Some(Command::None)
        }

        Action::HideModal => {
            state.ui_state.active_modal = None;
            Some(Command::None)
        }

        Action::ModalConfirm => Some(handle_modal_confirm(state)),

        Action::ModalCancel => {
            state.ui_state.active_modal = None;
            Some(Command::None)
        }

        Action::UpdateModalInput(v) => {
            if let Some(ModalType::Input { value, .. }) = &mut state.ui_state.active_modal {
                *value = v.clone();
            }
            Some(Command::None)
        }

        Action::UpdateConnectionForm(f) => {
            if let Some(ModalType::ConnectionForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        Action::UpdateTopicCreateForm(f) => {
            if let Some(ModalType::TopicCreateForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        Action::UpdateProduceForm(f) => {
            if let Some(ModalType::ProduceForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        Action::ShowToast { message, level } => {
            toast(state, message, *level);
            Some(Command::None)
        }

        Action::DismissToast(id) => {
            state.ui_state.toast_messages.retain(|t| t.id != *id);
            Some(Command::None)
        }

        _ => None,
    }
}

/// Add a toast message to the UI state and log it.
pub fn toast(state: &mut AppState, msg: &str, level: Level) {
    state.ui_state.toast_messages.push(ToastMessage {
        id: Uuid::new_v4(),
        message: msg.into(),
        level,
        created_at: Utc::now(),
    });
    state.logs_state.add(level, msg.into());
}

/// Remove expired toast messages.
pub fn expire_toasts(toasts: &mut Vec<ToastMessage>) {
    let now = Utc::now();
    toasts.retain(|t| now.signed_duration_since(t.created_at) < chrono::Duration::seconds(5));
}

fn handle_modal_confirm(state: &mut AppState) -> Command {
    let Some(modal) = state.ui_state.active_modal.take() else {
        return Command::None;
    };

    match modal {
        ModalType::Confirm { action, .. } => match action {
            ConfirmAction::DeleteTopic(n) => Command::DeleteKafkaTopic(n),
            ConfirmAction::DeleteConnection(id) => Command::DeleteConnectionProfile(id),
            ConfirmAction::DisconnectCluster => Command::DisconnectFromKafka,
        },
        ModalType::Input { action, value, .. } => match action {
            InputAction::FilterTopics => {
                state.topics_state.filter = value;
                state.topics_state.selected_index = 0;
                Command::None
            }
            InputAction::FilterConsumerGroups => {
                state.consumer_groups_state.filter = value;
                state.consumer_groups_state.selected_index = 0;
                Command::None
            }
            InputAction::ProduceMessage { topic } => Command::ProduceKafkaMessage {
                topic,
                key: None,
                value,
                headers: Default::default(),
            },
            InputAction::CreateTopic => Command::CreateKafkaTopic {
                name: value,
                partitions: 1,
                replication_factor: 1,
            },
        },
        ModalType::ConnectionForm(f) => {
            let auth = match f.auth_type {
                AuthType::None => AuthConfig::None,
                AuthType::SaslPlain => AuthConfig::SaslPlain {
                    username: f.username,
                    password: f.password,
                },
                AuthType::SaslScram256 => AuthConfig::SaslScram256 {
                    username: f.username,
                    password: f.password,
                },
                AuthType::SaslScram512 => AuthConfig::SaslScram512 {
                    username: f.username,
                    password: f.password,
                },
            };
            let consumer_group = if f.consumer_group.is_empty() {
                None
            } else {
                Some(f.consumer_group)
            };
            let profile = ConnectionProfile {
                id: Uuid::new_v4(),
                name: f.name,
                brokers: f.brokers,
                consumer_group,
                auth,
                created_at: Utc::now(),
                last_used: None,
            };
            state.connection.status = ConnectionStatus::Connecting;
            state.connection.active_profile = Some(profile.clone());
            Command::ConnectToKafka(profile)
        }
        ModalType::TopicCreateForm(f) => {
            match (
                parse_partitions(&f.partitions),
                parse_replication_factor(&f.replication_factor),
            ) {
                (Ok(partitions), Ok(replication_factor)) => Command::CreateKafkaTopic {
                    name: f.name,
                    partitions,
                    replication_factor,
                },
                (Err(e), _) | (_, Err(e)) => {
                    toast(state, &e.to_string(), Level::Error);
                    state.ui_state.active_modal = Some(ModalType::TopicCreateForm(f));
                    Command::None
                }
            }
        }
        ModalType::ProduceForm(f) => Command::ProduceKafkaMessage {
            topic: f.topic,
            key: if f.key.is_empty() { None } else { Some(f.key) },
            value: f.value,
            headers: Default::default(),
        },
        ModalType::AddPartitionsForm(f) => {
            match parse_new_partition_count(&f.new_count, f.current_count) {
                Ok(new_count) => Command::AddTopicPartitions {
                    topic: f.topic,
                    new_count,
                },
                Err(e) => {
                    toast(state, &e.to_string(), Level::Error);
                    state.ui_state.active_modal = Some(ModalType::AddPartitionsForm(f));
                    Command::None
                }
            }
        }
        ModalType::AlterConfigForm(f) => {
            let configs = f.modified_configs();
            if configs.is_empty() {
                Command::None
            } else {
                Command::AlterKafkaTopicConfig {
                    topic: f.topic,
                    configs,
                }
            }
        }
        ModalType::PurgeTopicForm(f) => {
            if f.purge_all {
                Command::PurgeKafkaTopic {
                    topic: f.topic,
                    before_offset: i64::MAX,
                }
            } else {
                match parse_offset(&f.offset) {
                    Ok(offset) => Command::PurgeKafkaTopic {
                        topic: f.topic,
                        before_offset: offset,
                    },
                    Err(e) => {
                        toast(state, &e.to_string(), Level::Error);
                        state.ui_state.active_modal = Some(ModalType::PurgeTopicForm(f));
                        Command::None
                    }
                }
            }
        }
    }
}
