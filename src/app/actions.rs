use std::collections::HashMap;

use crate::app::state::{
    ConnectionFormState, ConnectionProfile, ConsumerGroupDetail, ConsumerGroupInfo, KafkaMessage,
    ModalType, OffsetMode, Screen, SidebarItem, ToastLevel, TopicCreateFormState, TopicInfo, TopicSortField,
};

/// All possible actions in the application (TEA Messages)
#[derive(Debug, Clone)]
pub enum Action {
    // === System Actions ===
    Tick,
    Quit,
    Resize(u16, u16),
    ClearError,

    // === Navigation Actions ===
    Navigate(Screen),
    GoBack,
    FocusSidebar,
    FocusContent,
    SelectSidebarItem(SidebarItem),

    // === Connection Actions ===
    Connect(ConnectionProfile),
    Disconnect,
    ConnectionSuccess,
    ConnectionFailed(String),
    LoadSavedConnections,
    ConnectionsLoaded(Vec<ConnectionProfile>),
    SaveConnection(ConnectionProfile),
    RequestDeleteConnection,
    DeleteConnection(uuid::Uuid),
    ConnectionDeleted(uuid::Uuid),

    // === Topic Actions ===
    FetchTopics,
    TopicsFetched(Vec<TopicInfo>),
    TopicsFetchFailed(String),
    SelectTopic(usize),
    FilterTopics(String),
    ClearTopicFilter,
    SortTopics(TopicSortField),
    CreateTopic {
        name: String,
        partitions: i32,
        replication_factor: i32,
    },
    TopicCreated {
        name: String,
        partitions: i32,
        replication_factor: i32,
    },
    TopicCreateFailed(String),
    DeleteTopic(String),
    TopicDeleted(String),
    TopicDeleteFailed(String),
    ViewTopicDetails(String),
    ViewTopicMessages(String),

    // === Message Actions ===
    FetchMessages {
        topic: String,
        offset_mode: OffsetMode,
        partition: Option<i32>,
    },
    MessagesFetched(Vec<KafkaMessage>),
    MessageReceived(KafkaMessage),
    MessagesFetchFailed(String),
    SelectMessage(usize),
    SetOffsetMode(OffsetMode),
    SetPartitionFilter(Option<i32>),
    StartConsuming {
        topic: String,
    },
    StopConsuming,
    ProduceMessage {
        topic: String,
        key: Option<String>,
        value: String,
        headers: HashMap<String, String>,
    },
    MessageProduced,
    MessageProduceFailed(String),
    ToggleMessageDetail,
    ClearMessages,

    // === Consumer Group Actions ===
    FetchConsumerGroups,
    ConsumerGroupsFetched(Vec<ConsumerGroupInfo>),
    ConsumerGroupsFetchFailed(String),
    SelectConsumerGroup(usize),
    FilterConsumerGroups(String),
    ClearConsumerGroupFilter,
    ViewConsumerGroupDetails(String),
    ConsumerGroupDetailsFetched(ConsumerGroupDetail),

    // === UI Actions ===
    ShowHelp,
    HideHelp,
    ShowModal(ModalType),
    HideModal,
    ModalConfirm,
    ModalCancel,
    UpdateModalInput(String),
    UpdateConnectionForm(ConnectionFormState),
    UpdateTopicCreateForm(TopicCreateFormState),
    ShowToast {
        message: String,
        level: ToastLevel,
    },
    DismissToast(uuid::Uuid),
    ClearToasts,

    // === Input Actions (List Navigation) ===
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    ScrollToTop,
    ScrollToBottom,
    Select,
    Cancel,
}

/// Commands represent side effects to be executed asynchronously
#[derive(Debug)]
pub enum Command {
    None,
    Batch(Vec<Command>),

    // === Kafka Operations ===
    ConnectToKafka(ConnectionProfile),
    DisconnectFromKafka,
    FetchTopicList,
    FetchTopicDetails(String),
    CreateKafkaTopic {
        name: String,
        partitions: i32,
        replication_factor: i32,
    },
    DeleteKafkaTopic(String),
    FetchMessages {
        topic: String,
        offset_mode: OffsetMode,
        partition: Option<i32>,
        limit: usize,
    },
    StartMessageConsumer {
        topic: String,
        offset_mode: OffsetMode,
        partition: Option<i32>,
    },
    StopMessageConsumer,
    ProduceKafkaMessage {
        topic: String,
        key: Option<String>,
        value: String,
        headers: HashMap<String, String>,
    },
    FetchConsumerGroupList,
    FetchConsumerGroupDetails(String),

    // === Database Operations ===
    LoadConnectionProfiles,
    SaveConnectionProfile(ConnectionProfile),
    DeleteConnectionProfile(uuid::Uuid),
    SaveToHistory(HistoryEntry),
    LoadHistory,

    // === Timer Operations ===
    ScheduleTick(std::time::Duration),
}

impl Command {
    pub fn batch(commands: impl IntoIterator<Item = Command>) -> Command {
        let cmds: Vec<Command> = commands.into_iter().collect();
        if cmds.is_empty() {
            Command::None
        } else if cmds.len() == 1 {
            cmds.into_iter().next().unwrap()
        } else {
            Command::Batch(cmds)
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub entry_type: HistoryType,
    pub connection_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum HistoryType {
    ProducedMessage {
        topic: String,
        key: Option<String>,
        value: String,
    },
    ConsumedMessage {
        topic: String,
        partition: i32,
        offset: i64,
    },
}
