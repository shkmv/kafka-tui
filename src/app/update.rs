use chrono::Utc;
use uuid::Uuid;

use crate::app::actions::{Action, Command};
use crate::app::state::*;

pub fn update(state: &mut AppState, action: Action) -> Command {
    match action {
        // System
        Action::Tick => { expire_toasts(&mut state.ui_state.toast_messages); Command::None }
        Action::Quit => { state.running = false; Command::None }
        Action::Resize(_, _) => Command::None,

        // Navigation
        Action::Navigate(screen) => {
            if state.active_screen == screen { return Command::None; }
            state.screen_history.push(state.active_screen.clone());
            state.active_screen = screen.clone();
            match screen {
                Screen::Topics => Command::FetchTopicList,
                Screen::ConsumerGroups => Command::FetchConsumerGroupList,
                Screen::Brokers => Command::FetchBrokerList,
                Screen::Messages { ref topic_name } => Command::FetchMessages {
                    topic: topic_name.clone(),
                    offset_mode: state.messages_state.offset_mode.clone(),
                    partition: state.messages_state.partition_filter,
                    limit: 100,
                },
                _ => Command::None,
            }
        }

        Action::GoBack => {
            if state.ui_state.active_modal.take().is_some() { return Command::None; }
            if std::mem::take(&mut state.ui_state.show_help) { return Command::None; }
            if let Some(prev) = state.screen_history.pop() { state.active_screen = prev; }
            Command::None
        }

        Action::FocusSidebar => { state.ui_state.sidebar_focused = true; Command::None }
        Action::FocusContent => { state.ui_state.sidebar_focused = false; Command::None }

        Action::SelectSidebarItem(item) => {
            state.ui_state.selected_sidebar_item = item.clone();
            update(state, Action::Navigate(item.to_screen()))
        }

        // Connection
        Action::Connect(profile) => {
            state.connection.status = ConnectionStatus::Connecting;
            state.connection.active_profile = Some(profile.clone());
            Command::ConnectToKafka(profile)
        }

        Action::ConnectionSuccess => {
            state.connection.status = ConnectionStatus::Connected;
            state.active_screen = Screen::Topics;
            toast(state, "Connected", ToastLevel::Success);
            let mut cmds = vec![Command::FetchTopicList, Command::FetchConsumerGroupList];
            if let Some(p) = &state.connection.active_profile {
                cmds.push(Command::SaveConnectionProfile(p.clone()));
            }
            Command::Batch(cmds)
        }

        Action::ConnectionFailed(e) => {
            state.connection.status = ConnectionStatus::Error(e.clone());
            state.connection.active_profile = None;
            toast(state, &format!("Connection failed: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::Disconnect => {
            state.connection = Default::default();
            state.topics_state = Default::default();
            state.messages_state = Default::default();
            state.consumer_groups_state = Default::default();
            state.active_screen = Screen::Welcome;
            state.screen_history.clear();
            Command::DisconnectFromKafka
        }

        Action::LoadSavedConnections => Command::LoadConnectionProfiles,
        Action::ConnectionsLoaded(p) => { state.connection.available_profiles = p; Command::None }
        Action::SaveConnection(p) => Command::SaveConnectionProfile(p),

        Action::RequestDeleteConnection => {
            if let Some(profile) = state.connection.available_profiles.get(state.connection.selected_index).cloned() {
                update(state, Action::ShowModal(ModalType::Confirm {
                    title: "Delete Connection".into(),
                    message: format!("Delete '{}'?", profile.name),
                    action: ConfirmAction::DeleteConnection(profile.id),
                }))
            } else { Command::None }
        }

        Action::DeleteConnection(id) => Command::DeleteConnectionProfile(id),

        Action::ConnectionDeleted(id) => {
            state.connection.available_profiles.retain(|p| p.id != id);
            state.connection.selected_index = state.connection.selected_index.min(
                state.connection.available_profiles.len().saturating_sub(1)
            );
            toast(state, "Connection deleted", ToastLevel::Success);
            Command::None
        }

        // Topics
        Action::FetchTopics => { state.topics_state.loading = true; Command::FetchTopicList }

        Action::TopicsFetched(topics) => {
            state.topics_state.topics = topics;
            state.topics_state.loading = false;
            state.topics_state.selected_index = 0;
            Command::None
        }

        Action::TopicsFetchFailed(e) => {
            state.topics_state.loading = false;
            toast(state, &format!("Failed to fetch topics: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::SelectTopic(i) => {
            if i < state.topics_state.filtered_topics().len() { state.topics_state.selected_index = i; }
            Command::None
        }

        Action::FilterTopics(f) => { state.topics_state.filter = f; state.topics_state.selected_index = 0; Command::None }
        Action::ClearTopicFilter => { state.topics_state.filter.clear(); state.topics_state.selected_index = 0; Command::None }

        Action::SortTopics(field) => {
            if state.topics_state.sort_by == field {
                state.topics_state.sort_ascending = !state.topics_state.sort_ascending;
            } else {
                state.topics_state.sort_by = field;
                state.topics_state.sort_ascending = true;
            }
            sort_topics(state);
            Command::None
        }

        Action::CreateTopic { name, partitions, replication_factor } =>
            Command::CreateKafkaTopic { name, partitions, replication_factor },

        Action::TopicCreated { name, partitions, replication_factor } => {
            state.ui_state.active_modal = None;
            state.topics_state.topics.push(TopicInfo {
                name: name.clone(), partition_count: partitions, replication_factor,
                message_count: Some(0), is_internal: false,
            });
            toast(state, &format!("Topic '{}' created", name), ToastLevel::Success);
            Command::None
        }

        Action::TopicCreateFailed(e) => { toast(state, &format!("Failed to create topic: {}", e), ToastLevel::Error); Command::None }
        Action::DeleteTopic(name) => Command::DeleteKafkaTopic(name),

        Action::TopicDeleted(name) => {
            state.ui_state.active_modal = None;
            toast(state, &format!("Topic '{}' deleted", name), ToastLevel::Success);
            Command::FetchTopicList
        }

        Action::TopicDeleteFailed(e) => { toast(state, &format!("Delete failed: {}", e), ToastLevel::Error); Command::None }

        Action::RequestViewTopicDetails => {
            state.topics_state.selected_topic().map(|t| t.name.clone())
                .map(|n| update(state, Action::ViewTopicDetails(n)))
                .unwrap_or(Command::None)
        }

        Action::ViewTopicDetails(name) => {
            state.screen_history.push(state.active_screen.clone());
            state.topics_state.current_detail = None;
            state.topics_state.detail_tab = TopicDetailTab::default();
            state.active_screen = Screen::TopicDetails { topic_name: name.clone() };
            Command::FetchTopicDetails(name)
        }

        Action::TopicDetailsFetched(detail) => { state.topics_state.current_detail = Some(detail); Command::None }
        Action::TopicDetailsFetchFailed(e) => { toast(state, &e, ToastLevel::Error); Command::None }

        Action::SwitchTopicDetailTab => {
            state.topics_state.detail_tab = match state.topics_state.detail_tab {
                TopicDetailTab::Partitions => TopicDetailTab::Config,
                TopicDetailTab::Config => TopicDetailTab::Partitions,
            };
            Command::None
        }

        Action::ViewTopicMessages(name) => {
            state.screen_history.push(state.active_screen.clone());
            state.messages_state.current_topic = Some(name.clone());
            state.messages_state.messages.clear();
            state.messages_state.selected_index = 0;
            state.active_screen = Screen::Messages { topic_name: name.clone() };
            Command::FetchMessages {
                topic: name,
                offset_mode: state.messages_state.offset_mode.clone(),
                partition: state.messages_state.partition_filter,
                limit: 100,
            }
        }

        // Topic Management
        Action::AddPartitions { topic, new_count } => Command::AddTopicPartitions { topic, new_count },

        Action::PartitionsAdded(topic) => {
            state.ui_state.active_modal = None;
            toast(state, &format!("Partitions added to '{}'", topic), ToastLevel::Success);
            Command::FetchTopicDetails(topic)
        }

        Action::PartitionsAddFailed(e) => {
            toast(state, &format!("Add partitions failed: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::AlterTopicConfig { topic, configs } => Command::AlterKafkaTopicConfig { topic, configs },

        Action::TopicConfigAltered(topic) => {
            state.ui_state.active_modal = None;
            toast(state, &format!("Config updated for '{}'", topic), ToastLevel::Success);
            Command::FetchTopicDetails(topic)
        }

        Action::TopicConfigAlterFailed(e) => {
            toast(state, &format!("Alter config failed: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::PurgeTopic { topic, before_offset } => Command::PurgeKafkaTopic { topic, before_offset },

        Action::TopicPurged(topic) => {
            state.ui_state.active_modal = None;
            toast(state, &format!("Messages purged from '{}'", topic), ToastLevel::Success);
            Command::FetchTopicDetails(topic)
        }

        Action::TopicPurgeFailed(e) => {
            toast(state, &format!("Purge failed: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::UpdateAddPartitionsForm(f) => {
            if let Some(ModalType::AddPartitionsForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        Action::UpdateAlterConfigForm(f) => {
            if let Some(ModalType::AlterConfigForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        Action::UpdatePurgeTopicForm(f) => {
            if let Some(ModalType::PurgeTopicForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        // Messages
        Action::FetchMessages { topic, offset_mode, partition } => {
            state.messages_state.loading = true;
            state.messages_state.offset_mode = offset_mode.clone();
            state.messages_state.partition_filter = partition;
            Command::FetchMessages { topic, offset_mode, partition, limit: 100 }
        }

        Action::MessagesFetched(msgs) => {
            state.messages_state.messages = msgs;
            state.messages_state.loading = false;
            state.messages_state.selected_index = 0;
            Command::None
        }

        Action::MessageReceived(msg) => { state.messages_state.messages.push(msg); Command::None }

        Action::MessagesFetchFailed(e) => {
            state.messages_state.loading = false;
            toast(state, &format!("Failed to fetch messages: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::SelectMessage(i) => {
            if i < state.messages_state.messages.len() { state.messages_state.selected_index = i; }
            Command::None
        }

        Action::SetOffsetMode(m) => { state.messages_state.offset_mode = m; Command::None }
        Action::SetPartitionFilter(p) => { state.messages_state.partition_filter = p; Command::None }

        Action::StartConsuming { topic } => {
            state.messages_state.consumer_running = true;
            Command::StartMessageConsumer {
                topic,
                offset_mode: state.messages_state.offset_mode.clone(),
                partition: state.messages_state.partition_filter,
            }
        }

        Action::StopConsuming => { state.messages_state.consumer_running = false; Command::StopMessageConsumer }

        Action::ProduceMessage { topic, key, value, headers } =>
            Command::ProduceKafkaMessage { topic, key, value, headers },

        Action::MessageProduced => {
            state.ui_state.active_modal = None;
            toast(state, "Message produced", ToastLevel::Success);
            if let Screen::Messages { topic_name } = &state.active_screen {
                Command::FetchMessages { topic: topic_name.clone(), offset_mode: OffsetMode::Latest, partition: None, limit: 100 }
            } else { Command::None }
        }

        Action::MessageProduceFailed(e) => { toast(state, &format!("Produce failed: {}", e), ToastLevel::Error); Command::None }
        Action::ToggleMessageDetail => { state.messages_state.detail_expanded = !state.messages_state.detail_expanded; Command::None }
        Action::ClearMessages => { state.messages_state.messages.clear(); state.messages_state.selected_index = 0; Command::None }

        // Consumer Groups
        Action::FetchConsumerGroups => { state.consumer_groups_state.loading = true; Command::FetchConsumerGroupList }

        Action::ConsumerGroupsFetched(groups) => {
            state.consumer_groups_state.groups = groups;
            state.consumer_groups_state.loading = false;
            state.consumer_groups_state.selected_index = 0;
            Command::None
        }

        Action::ConsumerGroupsFetchFailed(e) => {
            state.consumer_groups_state.loading = false;
            toast(state, &format!("Failed to fetch groups: {}", e), ToastLevel::Error);
            Command::None
        }

        Action::SelectConsumerGroup(i) => {
            if i < state.consumer_groups_state.filtered_groups().len() { state.consumer_groups_state.selected_index = i; }
            Command::None
        }

        Action::FilterConsumerGroups(f) => { state.consumer_groups_state.filter = f; state.consumer_groups_state.selected_index = 0; Command::None }
        Action::ClearConsumerGroupFilter => { state.consumer_groups_state.filter.clear(); state.consumer_groups_state.selected_index = 0; Command::None }

        Action::ViewConsumerGroupDetails(id) => {
            state.screen_history.push(state.active_screen.clone());
            state.consumer_groups_state.current_detail = None;
            state.consumer_groups_state.detail_tab = ConsumerGroupDetailTab::default();
            state.active_screen = Screen::ConsumerGroupDetails { group_id: id.clone() };
            Command::FetchConsumerGroupDetails(id)
        }

        Action::ConsumerGroupDetailsFetched(detail) => { state.consumer_groups_state.current_detail = Some(detail); Command::None }
        Action::ConsumerGroupDetailsFetchFailed(e) => { toast(state, &e, ToastLevel::Error); Command::None }

        Action::SwitchConsumerGroupDetailTab => {
            state.consumer_groups_state.detail_tab = match state.consumer_groups_state.detail_tab {
                ConsumerGroupDetailTab::Members => ConsumerGroupDetailTab::Offsets,
                ConsumerGroupDetailTab::Offsets => ConsumerGroupDetailTab::Members,
            };
            Command::None
        }

        // Brokers
        Action::FetchBrokers => { state.brokers_state.loading = true; Command::FetchBrokerList }

        Action::BrokersFetched { brokers, cluster_id } => {
            state.brokers_state.brokers = brokers;
            state.brokers_state.cluster_id = cluster_id;
            state.brokers_state.loading = false;
            Command::None
        }

        Action::BrokersFetchFailed(e) => {
            state.brokers_state.loading = false;
            toast(state, &format!("Failed to fetch brokers: {}", e), ToastLevel::Error);
            Command::None
        }

        // UI
        Action::ShowHelp => { state.ui_state.show_help = true; Command::None }
        Action::HideHelp => { state.ui_state.show_help = false; Command::None }
        Action::ShowModal(m) => { state.ui_state.active_modal = Some(m); Command::None }
        Action::HideModal => { state.ui_state.active_modal = None; Command::None }
        Action::ModalConfirm => handle_modal_confirm(state),
        Action::ModalCancel => { state.ui_state.active_modal = None; Command::None }

        Action::UpdateModalInput(v) => {
            if let Some(ModalType::Input { value, .. }) = &mut state.ui_state.active_modal { *value = v; }
            Command::None
        }

        Action::UpdateConnectionForm(f) => {
            if let Some(ModalType::ConnectionForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        Action::UpdateTopicCreateForm(f) => {
            if let Some(ModalType::TopicCreateForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        Action::UpdateProduceForm(f) => {
            if let Some(ModalType::ProduceForm(s)) = &mut state.ui_state.active_modal { *s = f; }
            Command::None
        }

        Action::ShowToast { message, level } => { toast(state, &message, level); Command::None }
        Action::DismissToast(id) => { state.ui_state.toast_messages.retain(|t| t.id != id); Command::None }

        // List Navigation
        Action::MoveUp => { nav_up(state); Command::None }
        Action::MoveDown => { nav_down(state); Command::None }
        Action::MoveLeft => { if state.ui_state.sidebar_focused { sidebar_prev(state); } Command::None }
        Action::MoveRight => { if state.ui_state.sidebar_focused { sidebar_next(state); } Command::None }
        Action::PageUp => { for _ in 0..10 { nav_up(state); } Command::None }
        Action::PageDown => { for _ in 0..10 { nav_down(state); } Command::None }
        Action::ScrollToTop => { nav_to(state, 0); Command::None }
        Action::ScrollToBottom => { nav_to(state, usize::MAX); Command::None }
        Action::Select => handle_select(state),
        Action::Cancel => update(state, Action::GoBack),
    }
}

fn toast(state: &mut AppState, msg: &str, level: ToastLevel) {
    state.ui_state.toast_messages.push(ToastMessage {
        id: Uuid::new_v4(), message: msg.into(), level, created_at: Utc::now(),
    });
}

fn expire_toasts(toasts: &mut Vec<ToastMessage>) {
    let now = Utc::now();
    toasts.retain(|t| now.signed_duration_since(t.created_at) < chrono::Duration::seconds(5));
}

fn sort_topics(state: &mut AppState) {
    let asc = state.topics_state.sort_ascending;
    state.topics_state.topics.sort_by(|a, b| {
        let cmp = match state.topics_state.sort_by {
            TopicSortField::Name => a.name.cmp(&b.name),
            TopicSortField::Partitions => a.partition_count.cmp(&b.partition_count),
            TopicSortField::Replication => a.replication_factor.cmp(&b.replication_factor),
        };
        if asc { cmp } else { cmp.reverse() }
    });
}

fn handle_modal_confirm(state: &mut AppState) -> Command {
    let Some(modal) = state.ui_state.active_modal.take() else { return Command::None };

    match modal {
        ModalType::Confirm { action, .. } => match action {
            ConfirmAction::DeleteTopic(n) => Command::DeleteKafkaTopic(n),
            ConfirmAction::DeleteConnection(id) => Command::DeleteConnectionProfile(id),
            ConfirmAction::DisconnectCluster => Command::DisconnectFromKafka,
        },
        ModalType::Input { action, value, .. } => match action {
            InputAction::FilterTopics => { state.topics_state.filter = value; state.topics_state.selected_index = 0; Command::None }
            InputAction::FilterConsumerGroups => { state.consumer_groups_state.filter = value; state.consumer_groups_state.selected_index = 0; Command::None }
            InputAction::ProduceMessage { topic } => Command::ProduceKafkaMessage { topic, key: None, value, headers: Default::default() },
            InputAction::CreateTopic => Command::CreateKafkaTopic { name: value, partitions: 1, replication_factor: 1 },
        },
        ModalType::ConnectionForm(f) => {
            let auth = match f.auth_type {
                AuthType::None => AuthConfig::None,
                AuthType::SaslPlain => AuthConfig::SaslPlain { username: f.username, password: f.password },
                AuthType::SaslScram256 => AuthConfig::SaslScram256 { username: f.username, password: f.password },
                AuthType::SaslScram512 => AuthConfig::SaslScram512 { username: f.username, password: f.password },
            };
            let profile = ConnectionProfile { id: Uuid::new_v4(), name: f.name, brokers: f.brokers, auth, created_at: Utc::now(), last_used: None };
            state.connection.status = ConnectionStatus::Connecting;
            state.connection.active_profile = Some(profile.clone());
            Command::ConnectToKafka(profile)
        }
        ModalType::TopicCreateForm(f) => Command::CreateKafkaTopic {
            name: f.name,
            partitions: f.partitions.parse().unwrap_or(1),
            replication_factor: f.replication_factor.parse().unwrap_or(1),
        },
        ModalType::ProduceForm(f) => Command::ProduceKafkaMessage {
            topic: f.topic,
            key: if f.key.is_empty() { None } else { Some(f.key) },
            value: f.value,
            headers: Default::default(),
        },
        ModalType::AddPartitionsForm(f) => {
            let new_count = f.new_count.parse().unwrap_or(f.current_count);
            if new_count > f.current_count {
                Command::AddTopicPartitions { topic: f.topic, new_count }
            } else {
                Command::None
            }
        }
        ModalType::AlterConfigForm(f) => {
            let configs = f.modified_configs();
            if configs.is_empty() {
                Command::None
            } else {
                Command::AlterKafkaTopicConfig { topic: f.topic, configs }
            }
        }
        ModalType::PurgeTopicForm(f) => {
            let offset = if f.purge_all { i64::MAX } else { f.offset.parse().unwrap_or(0) };
            Command::PurgeKafkaTopic { topic: f.topic, before_offset: offset }
        }
    }
}

fn nav_up(state: &mut AppState) {
    if state.ui_state.sidebar_focused { return sidebar_prev(state); }
    match &state.active_screen {
        Screen::Topics => state.topics_state.selected_index = state.topics_state.selected_index.saturating_sub(1),
        Screen::Messages { .. } => state.messages_state.selected_index = state.messages_state.selected_index.saturating_sub(1),
        Screen::ConsumerGroups => state.consumer_groups_state.selected_index = state.consumer_groups_state.selected_index.saturating_sub(1),
        Screen::Welcome => state.connection.selected_index = state.connection.selected_index.saturating_sub(1),
        _ => {}
    }
}

fn nav_down(state: &mut AppState) {
    if state.ui_state.sidebar_focused { return sidebar_next(state); }
    match &state.active_screen {
        Screen::Topics => {
            let max = state.topics_state.filtered_topics().len();
            if state.topics_state.selected_index + 1 < max { state.topics_state.selected_index += 1; }
        }
        Screen::Messages { .. } => {
            let max = state.messages_state.messages.len();
            if state.messages_state.selected_index + 1 < max { state.messages_state.selected_index += 1; }
        }
        Screen::ConsumerGroups => {
            let max = state.consumer_groups_state.filtered_groups().len();
            if state.consumer_groups_state.selected_index + 1 < max { state.consumer_groups_state.selected_index += 1; }
        }
        Screen::Welcome => {
            let max = state.connection.available_profiles.len();
            if state.connection.selected_index + 1 < max { state.connection.selected_index += 1; }
        }
        _ => {}
    }
}

fn nav_to(state: &mut AppState, target: usize) {
    match &state.active_screen {
        Screen::Topics => {
            let max = state.topics_state.filtered_topics().len().saturating_sub(1);
            state.topics_state.selected_index = target.min(max);
        }
        Screen::Messages { .. } => {
            let max = state.messages_state.messages.len().saturating_sub(1);
            state.messages_state.selected_index = target.min(max);
        }
        Screen::ConsumerGroups => {
            let max = state.consumer_groups_state.filtered_groups().len().saturating_sub(1);
            state.consumer_groups_state.selected_index = target.min(max);
        }
        _ => {}
    }
}

fn sidebar_prev(state: &mut AppState) {
    state.ui_state.selected_sidebar_item = match state.ui_state.selected_sidebar_item {
        SidebarItem::Topics => SidebarItem::Brokers,
        SidebarItem::ConsumerGroups => SidebarItem::Topics,
        SidebarItem::Brokers => SidebarItem::ConsumerGroups,
    };
}

fn sidebar_next(state: &mut AppState) {
    state.ui_state.selected_sidebar_item = match state.ui_state.selected_sidebar_item {
        SidebarItem::Topics => SidebarItem::ConsumerGroups,
        SidebarItem::ConsumerGroups => SidebarItem::Brokers,
        SidebarItem::Brokers => SidebarItem::Topics,
    };
}

fn handle_select(state: &mut AppState) -> Command {
    if state.ui_state.sidebar_focused {
        let item = state.ui_state.selected_sidebar_item.clone();
        state.ui_state.sidebar_focused = false;
        return update(state, Action::Navigate(item.to_screen()));
    }

    match &state.active_screen {
        Screen::Topics => {
            let name = state.topics_state.selected_topic().map(|t| t.name.clone());
            name.map(|n| update(state, Action::ViewTopicMessages(n))).unwrap_or(Command::None)
        }
        Screen::Messages { .. } => update(state, Action::ToggleMessageDetail),
        Screen::ConsumerGroups => {
            let id = state.consumer_groups_state.selected_group().map(|g| g.group_id.clone());
            id.map(|i| update(state, Action::ViewConsumerGroupDetails(i))).unwrap_or(Command::None)
        }
        Screen::Welcome => {
            let profile = state.connection.available_profiles.get(state.connection.selected_index).cloned();
            profile.map(|p| update(state, Action::Connect(p))).unwrap_or(Command::None)
        }
        _ => Command::None,
    }
}
