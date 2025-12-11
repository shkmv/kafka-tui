use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use uuid::Uuid;

/// Root application state - the single source of truth
#[derive(Debug, Default)]
pub struct AppState {
    /// Current active screen
    pub active_screen: Screen,

    /// Navigation history for back functionality
    pub screen_history: Vec<Screen>,

    /// Connection state
    pub connection: ConnectionState,

    /// Screen-specific states
    pub topics_state: TopicsState,
    pub messages_state: MessagesState,
    pub consumer_groups_state: ConsumerGroupsState,

    /// Global UI state
    pub ui_state: UiState,

    /// Application running flag
    pub running: bool,

    /// Last error (for display)
    pub last_error: Option<String>,

    /// Pending async operations
    pub pending_operations: HashMap<Uuid, PendingOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Display)]
pub enum Screen {
    #[default]
    Welcome,
    Topics,
    TopicDetails {
        topic_name: String,
    },
    TopicCreate,
    Messages {
        topic_name: String,
    },
    MessageProducer {
        topic_name: String,
    },
    ConsumerGroups,
    ConsumerGroupDetails {
        group_id: String,
    },
}

// === Connection State ===

#[derive(Debug, Default)]
pub struct ConnectionState {
    pub status: ConnectionStatus,
    pub active_profile: Option<ConnectionProfile>,
    pub available_profiles: Vec<ConnectionProfile>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub id: Uuid,
    pub name: String,
    pub brokers: String,
    pub auth: AuthConfig,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

impl Default for ConnectionProfile {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            brokers: String::new(),
            auth: AuthConfig::None,
            created_at: Utc::now(),
            last_used: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    #[default]
    None,
    SaslPlain {
        username: String,
        password: String,
    },
    SaslScram256 {
        username: String,
        password: String,
    },
    SaslScram512 {
        username: String,
        password: String,
    },
    Ssl {
        ca_location: Option<String>,
        cert_location: Option<String>,
        key_location: Option<String>,
        key_password: Option<String>,
    },
    SaslSsl {
        mechanism: SaslMechanism,
        username: String,
        password: String,
        ca_location: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaslMechanism {
    #[default]
    Plain,
    ScramSha256,
    ScramSha512,
}

// === Topics State ===

#[derive(Debug, Default)]
pub struct TopicsState {
    pub topics: Vec<TopicInfo>,
    pub selected_index: usize,
    pub filter: String,
    pub loading: bool,
    pub sort_by: TopicSortField,
    pub sort_ascending: bool,
}

impl TopicsState {
    pub fn filtered_topics(&self) -> Vec<&TopicInfo> {
        if self.filter.is_empty() {
            self.topics.iter().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.topics
                .iter()
                .filter(|t| t.name.to_lowercase().contains(&filter_lower))
                .collect()
        }
    }

    pub fn selected_topic(&self) -> Option<&TopicInfo> {
        let filtered = self.filtered_topics();
        filtered.get(self.selected_index).copied()
    }
}

#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub name: String,
    pub partition_count: i32,
    pub replication_factor: i32,
    pub message_count: Option<i64>,
    pub is_internal: bool,
}

#[derive(Debug, Clone, Default, EnumIter, Display, PartialEq, Eq)]
pub enum TopicSortField {
    #[default]
    Name,
    Partitions,
    Replication,
}

// === Messages State ===

#[derive(Debug, Default)]
pub struct MessagesState {
    pub messages: Vec<KafkaMessage>,
    pub selected_index: usize,
    pub partition_filter: Option<i32>,
    pub offset_mode: OffsetMode,
    pub loading: bool,
    pub consumer_running: bool,
    pub detail_expanded: bool,
    pub current_topic: Option<String>,
}

impl MessagesState {
    pub fn selected_message(&self) -> Option<&KafkaMessage> {
        self.messages.get(self.selected_index)
    }
}

#[derive(Debug, Clone)]
pub struct KafkaMessage {
    pub partition: i32,
    pub offset: i64,
    pub timestamp: Option<DateTime<Utc>>,
    pub key: Option<String>,
    pub value: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum OffsetMode {
    #[default]
    Latest,
    Earliest,
    Specific(i64),
    Timestamp(DateTime<Utc>),
}

// === Consumer Groups State ===

#[derive(Debug, Default)]
pub struct ConsumerGroupsState {
    pub groups: Vec<ConsumerGroupInfo>,
    pub selected_index: usize,
    pub filter: String,
    pub loading: bool,
}

impl ConsumerGroupsState {
    pub fn filtered_groups(&self) -> Vec<&ConsumerGroupInfo> {
        if self.filter.is_empty() {
            self.groups.iter().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.groups
                .iter()
                .filter(|g| g.group_id.to_lowercase().contains(&filter_lower))
                .collect()
        }
    }

    pub fn selected_group(&self) -> Option<&ConsumerGroupInfo> {
        let filtered = self.filtered_groups();
        filtered.get(self.selected_index).copied()
    }
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupInfo {
    pub group_id: String,
    pub state: String,
    pub members_count: usize,
    pub topics: Vec<String>,
    pub total_lag: i64,
}

#[derive(Debug, Clone)]
pub struct ConsumerGroupDetail {
    pub group_id: String,
    pub state: String,
    pub coordinator: Option<BrokerInfo>,
    pub members: Vec<GroupMember>,
    pub offsets: Vec<PartitionOffset>,
}

#[derive(Debug, Clone)]
pub struct BrokerInfo {
    pub id: i32,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
    pub assignments: Vec<TopicPartition>,
}

#[derive(Debug, Clone)]
pub struct TopicPartition {
    pub topic: String,
    pub partition: i32,
}

#[derive(Debug, Clone)]
pub struct PartitionOffset {
    pub topic: String,
    pub partition: i32,
    pub current_offset: i64,
    pub log_end_offset: i64,
    pub lag: i64,
}

// === UI State ===

#[derive(Debug, Default)]
pub struct UiState {
    pub show_help: bool,
    pub active_modal: Option<ModalType>,
    pub toast_messages: Vec<ToastMessage>,
    pub sidebar_focused: bool,
    pub selected_sidebar_item: SidebarItem,
}

#[derive(Debug, Clone, Default, EnumIter, Display, PartialEq, Eq)]
pub enum SidebarItem {
    #[default]
    Topics,
    ConsumerGroups,
    Brokers,
}

impl SidebarItem {
    pub fn to_screen(&self) -> Screen {
        match self {
            SidebarItem::Topics => Screen::Topics,
            SidebarItem::ConsumerGroups => Screen::ConsumerGroups,
            SidebarItem::Brokers => Screen::Topics, // TODO: Implement Brokers screen
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModalType {
    Confirm {
        title: String,
        message: String,
        action: ConfirmAction,
    },
    Input {
        title: String,
        placeholder: String,
        value: String,
        action: InputAction,
    },
    ConnectionForm(ConnectionFormState),
    TopicCreateForm(TopicCreateFormState),
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionFormState {
    pub name: String,
    pub brokers: String,
    pub auth_type: AuthType,
    pub username: String,
    pub password: String,
    pub focused_field: ConnectionFormField,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConnectionFormField {
    #[default]
    Name,
    Brokers,
    AuthType,
    Username,
    Password,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum AuthType {
    #[default]
    None,
    SaslPlain,
    SaslScram256,
    SaslScram512,
}

impl AuthType {
    pub fn display_name(&self) -> &'static str {
        match self {
            AuthType::None => "None",
            AuthType::SaslPlain => "SASL/PLAIN",
            AuthType::SaslScram256 => "SASL/SCRAM-256",
            AuthType::SaslScram512 => "SASL/SCRAM-512",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            AuthType::None => AuthType::SaslPlain,
            AuthType::SaslPlain => AuthType::SaslScram256,
            AuthType::SaslScram256 => AuthType::SaslScram512,
            AuthType::SaslScram512 => AuthType::None,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            AuthType::None => AuthType::SaslScram512,
            AuthType::SaslPlain => AuthType::None,
            AuthType::SaslScram256 => AuthType::SaslPlain,
            AuthType::SaslScram512 => AuthType::SaslScram256,
        }
    }

    pub fn requires_credentials(&self) -> bool {
        !matches!(self, AuthType::None)
    }
}

#[derive(Debug, Clone)]
pub struct TopicCreateFormState {
    pub name: String,
    pub partitions: String,
    pub replication_factor: String,
    pub focused_field: TopicCreateFormField,
}

impl Default for TopicCreateFormState {
    fn default() -> Self {
        Self {
            name: String::new(),
            partitions: "1".to_string(),
            replication_factor: "1".to_string(),
            focused_field: TopicCreateFormField::Name,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TopicCreateFormField {
    #[default]
    Name,
    Partitions,
    ReplicationFactor,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeleteTopic(String),
    DisconnectCluster,
}

#[derive(Debug, Clone)]
pub enum InputAction {
    FilterTopics,
    FilterConsumerGroups,
    ProduceMessage { topic: String },
    CreateTopic,
}

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub id: Uuid,
    pub message: String,
    pub level: ToastLevel,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

// === Async Operations ===

#[derive(Debug)]
pub struct PendingOperation {
    pub operation_type: OperationType,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Connecting,
    FetchTopics,
    FetchMessages(String),
    FetchConsumerGroups,
    CreateTopic,
    DeleteTopic,
    ProduceMessage,
}
