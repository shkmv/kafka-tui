//! Topic-related action handlers.

use crate::app::actions::{Action, Command};
use crate::app::state::{
    AppState, Level, ModalType, Screen, TopicDetailTab, TopicInfo, TopicSortField,
};

use super::super::update::toast;

/// Handle topic actions.
pub fn handle(state: &mut AppState, action: &Action) -> Option<Command> {
    match action {
        Action::FetchTopics => {
            state.topics_state.loading = true;
            Some(Command::FetchTopicList)
        }

        Action::TopicsFetched(topics) => {
            state.topics_state.topics = topics.clone();
            state.topics_state.loading = false;
            state.topics_state.selected_index = 0;
            Some(Command::None)
        }

        Action::TopicsFetchFailed(e) => {
            state.topics_state.loading = false;
            toast(state, &format!("Failed to fetch topics: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::SelectTopic(i) => {
            if *i < state.topics_state.filtered_topics().len() {
                state.topics_state.selected_index = *i;
            }
            Some(Command::None)
        }

        Action::FilterTopics(f) => {
            state.topics_state.filter = f.clone();
            state.topics_state.selected_index = 0;
            Some(Command::None)
        }

        Action::ClearTopicFilter => {
            state.topics_state.filter.clear();
            state.topics_state.selected_index = 0;
            Some(Command::None)
        }

        Action::SortTopics(field) => {
            if state.topics_state.sort_by == *field {
                state.topics_state.sort_ascending = !state.topics_state.sort_ascending;
            } else {
                state.topics_state.sort_by = field.clone();
                state.topics_state.sort_ascending = true;
            }
            sort_topics(state);
            Some(Command::None)
        }

        Action::CreateTopic {
            name,
            partitions,
            replication_factor,
        } => Some(Command::CreateKafkaTopic {
            name: name.clone(),
            partitions: *partitions,
            replication_factor: *replication_factor,
        }),

        Action::TopicCreated {
            name,
            partitions,
            replication_factor,
        } => {
            state.ui_state.active_modal = None;
            state.topics_state.topics.push(TopicInfo {
                name: name.clone(),
                partition_count: *partitions,
                replication_factor: *replication_factor,
                message_count: Some(0),
                is_internal: false,
            });
            toast(state, &format!("Topic '{}' created", name), Level::Success);
            Some(Command::None)
        }

        Action::TopicCreateFailed(e) => {
            toast(state, &format!("Failed to create topic: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::DeleteTopic(name) => Some(Command::DeleteKafkaTopic(name.clone())),

        Action::TopicDeleted(name) => {
            state.ui_state.active_modal = None;
            toast(state, &format!("Topic '{}' deleted", name), Level::Success);
            Some(Command::FetchTopicList)
        }

        Action::TopicDeleteFailed(e) => {
            toast(state, &format!("Delete failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::RequestViewTopicDetails => {
            state
                .topics_state
                .selected_topic()
                .map(|t| t.name.clone())
                .map(|n| {
                    state.screen_history.push(state.active_screen.clone());
                    state.topics_state.current_detail = None;
                    state.topics_state.detail_tab = TopicDetailTab::default();
                    state.active_screen = Screen::TopicDetails {
                        topic_name: n.clone(),
                    };
                    Command::FetchTopicDetails(n)
                })
                .or(Some(Command::None))
        }

        Action::ViewTopicDetails(name) => {
            state.screen_history.push(state.active_screen.clone());
            state.topics_state.current_detail = None;
            state.topics_state.detail_tab = TopicDetailTab::default();
            state.active_screen = Screen::TopicDetails {
                topic_name: name.clone(),
            };
            Some(Command::FetchTopicDetails(name.clone()))
        }

        Action::TopicDetailsFetched(detail) => {
            state.topics_state.current_detail = Some(detail.clone());
            Some(Command::None)
        }

        Action::TopicDetailsFetchFailed(e) => {
            toast(state, e, Level::Error);
            Some(Command::None)
        }

        Action::SwitchTopicDetailTab => {
            state.topics_state.detail_tab = match state.topics_state.detail_tab {
                TopicDetailTab::Partitions => TopicDetailTab::Config,
                TopicDetailTab::Config => TopicDetailTab::Partitions,
            };
            Some(Command::None)
        }

        Action::ViewTopicMessages(name) => {
            state.screen_history.push(state.active_screen.clone());
            state.messages_state.current_topic = Some(name.clone());
            state.messages_state.messages.clear();
            state.messages_state.selected_index = 0;
            state.active_screen = Screen::Messages {
                topic_name: name.clone(),
            };
            Some(Command::FetchMessages {
                topic: name.clone(),
                offset_mode: state.messages_state.offset_mode.clone(),
                partition: state.messages_state.partition_filter,
                limit: 100,
            })
        }

        Action::AddPartitions { topic, new_count } => Some(Command::AddTopicPartitions {
            topic: topic.clone(),
            new_count: *new_count,
        }),

        Action::PartitionsAdded(topic) => {
            state.ui_state.active_modal = None;
            toast(
                state,
                &format!("Partitions added to '{}'", topic),
                Level::Success,
            );
            Some(Command::FetchTopicDetails(topic.clone()))
        }

        Action::PartitionsAddFailed(e) => {
            toast(state, &format!("Add partitions failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::AlterTopicConfig { topic, configs } => Some(Command::AlterKafkaTopicConfig {
            topic: topic.clone(),
            configs: configs.clone(),
        }),

        Action::TopicConfigAltered(topic) => {
            state.ui_state.active_modal = None;
            toast(
                state,
                &format!("Config updated for '{}'", topic),
                Level::Success,
            );
            Some(Command::FetchTopicDetails(topic.clone()))
        }

        Action::TopicConfigAlterFailed(e) => {
            toast(state, &format!("Alter config failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::PurgeTopic {
            topic,
            before_offset,
        } => Some(Command::PurgeKafkaTopic {
            topic: topic.clone(),
            before_offset: *before_offset,
        }),

        Action::TopicPurged(topic) => {
            state.ui_state.active_modal = None;
            toast(
                state,
                &format!("Messages purged from '{}'", topic),
                Level::Success,
            );
            Some(Command::FetchTopicDetails(topic.clone()))
        }

        Action::TopicPurgeFailed(e) => {
            toast(state, &format!("Purge failed: {}", e), Level::Error);
            Some(Command::None)
        }

        Action::UpdateAddPartitionsForm(f) => {
            if let Some(ModalType::AddPartitionsForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        Action::UpdateAlterConfigForm(f) => {
            if let Some(ModalType::AlterConfigForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        Action::UpdatePurgeTopicForm(f) => {
            if let Some(ModalType::PurgeTopicForm(s)) = &mut state.ui_state.active_modal {
                *s = f.clone();
            }
            Some(Command::None)
        }

        _ => None,
    }
}

fn sort_topics(state: &mut AppState) {
    let asc = state.topics_state.sort_ascending;
    state.topics_state.topics.sort_by(|a, b| {
        let cmp = match state.topics_state.sort_by {
            TopicSortField::Name => a.name.cmp(&b.name),
            TopicSortField::Partitions => a.partition_count.cmp(&b.partition_count),
            TopicSortField::Replication => a.replication_factor.cmp(&b.replication_factor),
        };
        if asc {
            cmp
        } else {
            cmp.reverse()
        }
    });
}
