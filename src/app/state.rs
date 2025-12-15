use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct AppState {
    pub active_screen: Screen,
    pub screen_history: Vec<Screen>,
    pub connection: ConnectionState,
    pub topics_state: TopicsState,
    pub messages_state: MessagesState,
    pub consumer_groups_state: ConsumerGroupsState,
    pub brokers_state: BrokersState,
    pub logs_state: LogsState,
    pub ui_state: UiState,
    pub running: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Welcome,
    Topics,
    TopicDetails { topic_name: String },
    Messages { topic_name: String },
    ConsumerGroups,
    ConsumerGroupDetails { group_id: String },
    Brokers,
    Logs,
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Welcome => write!(f, "Welcome"),
            Self::Topics => write!(f, "Topics"),
            Self::TopicDetails { topic_name } => write!(f, "Topic: {}", topic_name),
            Self::Messages { topic_name } => write!(f, "Messages: {}", topic_name),
            Self::ConsumerGroups => write!(f, "Consumer Groups"),
            Self::ConsumerGroupDetails { group_id } => write!(f, "Group: {}", group_id),
            Self::Brokers => write!(f, "Brokers"),
            Self::Logs => write!(f, "Logs"),
        }
    }
}

// === Connection ===

#[derive(Debug, Default)]
pub struct ConnectionState {
    pub status: ConnectionStatus,
    pub active_profile: Option<ConnectionProfile>,
    pub available_profiles: Vec<ConnectionProfile>,
    pub selected_index: usize,
}

impl Navigable for ConnectionState {
    fn selected_index(&self) -> usize { self.selected_index }
    fn set_selected_index(&mut self, index: usize) { self.selected_index = index; }
    fn item_count(&self) -> usize { self.available_profiles.len() }
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
    #[serde(default)]
    pub consumer_group: Option<String>,
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
            consumer_group: None,
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
    SaslPlain { username: String, password: String },
    SaslScram256 { username: String, password: String },
    SaslScram512 { username: String, password: String },
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

// === Topics ===

#[derive(Debug, Default)]
pub struct TopicsState {
    pub topics: Vec<TopicInfo>,
    pub selected_index: usize,
    pub filter: String,
    pub loading: bool,
    pub sort_by: TopicSortField,
    pub sort_ascending: bool,
    pub current_detail: Option<TopicDetail>,
    pub detail_tab: TopicDetailTab,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TopicDetailTab {
    #[default]
    Partitions,
    Config,
}

impl TopicsState {
    pub fn filtered_topics(&self) -> Vec<&TopicInfo> {
        if self.filter.is_empty() {
            self.topics.iter().collect()
        } else {
            let f = self.filter.to_lowercase();
            self.topics.iter().filter(|t| t.name.to_lowercase().contains(&f)).collect()
        }
    }

    pub fn selected_topic(&self) -> Option<&TopicInfo> {
        self.filtered_topics().get(self.selected_index).copied()
    }
}

impl Navigable for TopicsState {
    fn selected_index(&self) -> usize { self.selected_index }
    fn set_selected_index(&mut self, index: usize) { self.selected_index = index; }
    fn item_count(&self) -> usize { self.filtered_topics().len() }
}

#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub name: String,
    pub partition_count: i32,
    pub replication_factor: i32,
    pub message_count: Option<i64>,
    pub is_internal: bool,
}

#[derive(Debug, Clone)]
pub struct TopicDetail {
    pub name: String,
    pub partitions: Vec<PartitionInfo>,
    pub config: Vec<(String, String)>,
    pub is_internal: bool,
}

#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub id: i32,
    pub leader: i32,
    pub replicas: Vec<i32>,
    pub isr: Vec<i32>,
    pub low_watermark: i64,
    pub high_watermark: i64,
}

impl PartitionInfo {
    pub fn message_count(&self) -> i64 {
        self.high_watermark - self.low_watermark
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TopicSortField {
    #[default]
    Name,
    Partitions,
    Replication,
}

// === Messages ===

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

impl Navigable for MessagesState {
    fn selected_index(&self) -> usize { self.selected_index }
    fn set_selected_index(&mut self, index: usize) { self.selected_index = index; }
    fn item_count(&self) -> usize { self.messages.len() }
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

// === Consumer Groups ===

#[derive(Debug, Default)]
pub struct ConsumerGroupsState {
    pub groups: Vec<ConsumerGroupInfo>,
    pub selected_index: usize,
    pub filter: String,
    pub loading: bool,
    pub current_detail: Option<ConsumerGroupDetail>,
    pub detail_tab: ConsumerGroupDetailTab,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConsumerGroupDetailTab {
    #[default]
    Members,
    Offsets,
}

impl ConsumerGroupsState {
    pub fn filtered_groups(&self) -> Vec<&ConsumerGroupInfo> {
        if self.filter.is_empty() {
            self.groups.iter().collect()
        } else {
            let f = self.filter.to_lowercase();
            self.groups.iter().filter(|g| g.group_id.to_lowercase().contains(&f)).collect()
        }
    }

    pub fn selected_group(&self) -> Option<&ConsumerGroupInfo> {
        self.filtered_groups().get(self.selected_index).copied()
    }
}

impl Navigable for ConsumerGroupsState {
    fn selected_index(&self) -> usize { self.selected_index }
    fn set_selected_index(&mut self, index: usize) { self.selected_index = index; }
    fn item_count(&self) -> usize { self.filtered_groups().len() }
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
    pub is_controller: bool,
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

// === Brokers ===

#[derive(Debug, Default)]
pub struct BrokersState {
    pub brokers: Vec<BrokerInfo>,
    pub selected_index: usize,
    pub loading: bool,
    pub cluster_id: Option<String>,
}

// === Logs ===

const MAX_LOG_ENTRIES: usize = 1000;

/// Trait for navigable lists that support selection.
pub trait Navigable {
    fn selected_index(&self) -> usize;
    fn set_selected_index(&mut self, index: usize);
    fn item_count(&self) -> usize;

    fn nav_up(&mut self) {
        let current = self.selected_index();
        self.set_selected_index(current.saturating_sub(1));
    }

    fn nav_down(&mut self) {
        let current = self.selected_index();
        let max = self.item_count();
        if current + 1 < max {
            self.set_selected_index(current + 1);
        }
    }

    fn nav_to(&mut self, target: usize) {
        let max = self.item_count().saturating_sub(1);
        self.set_selected_index(target.min(max));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Success,
    Warning,
    Error,
}

impl Level {
    pub fn style(&self) -> ratatui::style::Style {
        use crate::ui::theme::THEME;
        match self {
            Level::Info => THEME.info_style(),
            Level::Success => THEME.success_style(),
            Level::Warning => THEME.warning_style(),
            Level::Error => THEME.error_style(),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Level::Info => "",
            Level::Success => "",
            Level::Warning => "",
            Level::Error => "",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Success => "OK",
            Level::Warning => "WARN",
            Level::Error => "ERR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: Level,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct LogsState {
    pub entries: VecDeque<LogEntry>,
    pub selected_index: usize,
    pub filter_level: Option<Level>,
}

impl LogsState {
    pub fn add(&mut self, level: Level, message: String) {
        self.entries.push_front(LogEntry {
            level,
            message,
            timestamp: Utc::now(),
        });
        if self.entries.len() > MAX_LOG_ENTRIES {
            self.entries.pop_back();
        }
    }

    pub fn filtered_entries(&self) -> Vec<&LogEntry> {
        match self.filter_level {
            Some(level) => self.entries.iter().filter(|e| e.level == level).collect(),
            None => self.entries.iter().collect(),
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.selected_index = 0;
    }
}

impl Navigable for LogsState {
    fn selected_index(&self) -> usize { self.selected_index }
    fn set_selected_index(&mut self, index: usize) { self.selected_index = index; }
    fn item_count(&self) -> usize { self.filtered_entries().len() }
}

// === UI ===

#[derive(Debug, Default)]
pub struct UiState {
    pub show_help: bool,
    pub active_modal: Option<ModalType>,
    pub toast_messages: Vec<ToastMessage>,
    pub sidebar_focused: bool,
    pub selected_sidebar_item: SidebarItem,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SidebarItem {
    #[default]
    Topics,
    ConsumerGroups,
    Brokers,
    Logs,
}

impl SidebarItem {
    pub const ALL: [SidebarItem; 4] = [Self::Topics, Self::ConsumerGroups, Self::Brokers, Self::Logs];

    pub fn to_screen(&self) -> Screen {
        match self {
            Self::Topics => Screen::Topics,
            Self::ConsumerGroups => Screen::ConsumerGroups,
            Self::Brokers => Screen::Brokers,
            Self::Logs => Screen::Logs,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Topics => "Topics",
            Self::ConsumerGroups => "Consumer Groups",
            Self::Brokers => "Brokers",
            Self::Logs => "Logs",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Topics => Self::ConsumerGroups,
            Self::ConsumerGroups => Self::Brokers,
            Self::Brokers => Self::Logs,
            Self::Logs => Self::Topics,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Topics => Self::Logs,
            Self::ConsumerGroups => Self::Topics,
            Self::Brokers => Self::ConsumerGroups,
            Self::Logs => Self::Brokers,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModalType {
    Confirm { title: String, message: String, action: ConfirmAction },
    Input { title: String, placeholder: String, value: String, action: InputAction },
    ConnectionForm(ConnectionFormState),
    TopicCreateForm(TopicCreateFormState),
    ProduceForm(ProduceFormState),
    AddPartitionsForm(AddPartitionsFormState),
    AlterConfigForm(AlterConfigFormState),
    PurgeTopicForm(PurgeTopicFormState),
}

#[derive(Debug, Clone, Default)]
pub struct ProduceFormState {
    pub topic: String,
    pub key: String,
    pub value: String,
    pub focused_field: ProduceFormField,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ProduceFormField {
    #[default]
    Key,
    Value,
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionFormState {
    pub name: String,
    pub brokers: String,
    pub consumer_group: String,
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
    ConsumerGroup,
    AuthType,
    Username,
    Password,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
            Self::None => "None",
            Self::SaslPlain => "SASL/PLAIN",
            Self::SaslScram256 => "SCRAM-256",
            Self::SaslScram512 => "SCRAM-512",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::SaslPlain,
            Self::SaslPlain => Self::SaslScram256,
            Self::SaslScram256 => Self::SaslScram512,
            Self::SaslScram512 => Self::None,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::None => Self::SaslScram512,
            Self::SaslPlain => Self::None,
            Self::SaslScram256 => Self::SaslPlain,
            Self::SaslScram512 => Self::SaslScram256,
        }
    }

    pub fn requires_credentials(&self) -> bool {
        !matches!(self, Self::None)
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
            partitions: "1".into(),
            replication_factor: "1".into(),
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
    DeleteConnection(Uuid),
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
    pub level: Level,
    pub created_at: DateTime<Utc>,
}

// === Topic Management Forms ===

#[derive(Debug, Clone)]
pub struct AddPartitionsFormState {
    pub topic: String,
    pub current_count: i32,
    pub new_count: String,
}

impl AddPartitionsFormState {
    pub fn new(topic: String, current_count: i32) -> Self {
        Self {
            topic,
            current_count,
            new_count: (current_count + 1).to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlterConfigFormState {
    pub topic: String,
    pub configs: Vec<(String, String, bool)>, // (key, value, modified)
    pub selected_index: usize,
    pub editing: bool,
    pub edit_value: String,
}

impl AlterConfigFormState {
    pub fn new(topic: String, configs: Vec<(String, String)>) -> Self {
        Self {
            topic,
            configs: configs.into_iter().map(|(k, v)| (k, v, false)).collect(),
            selected_index: 0,
            editing: false,
            edit_value: String::new(),
        }
    }

    pub fn modified_configs(&self) -> Vec<(String, String)> {
        self.configs.iter()
            .filter(|(_, _, modified)| *modified)
            .map(|(k, v, _)| (k.clone(), v.clone()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct PurgeTopicFormState {
    pub topic: String,
    pub offset: String,
    pub purge_all: bool,
}

impl PurgeTopicFormState {
    pub fn new(topic: String) -> Self {
        Self {
            topic,
            offset: String::new(),
            purge_all: true,
        }
    }
}
