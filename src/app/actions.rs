use std::collections::HashMap;

use crate::app::state::{
    AddPartitionsFormState, AlterConfigFormState, BrokerInfo, ConnectionFormState, ConnectionProfile,
    ConsumerGroupDetail, ConsumerGroupInfo, KafkaMessage, ModalType, OffsetMode, ProduceFormState,
    PurgeTopicFormState, Screen, SidebarItem, ToastLevel, TopicCreateFormState, TopicDetail,
    TopicInfo, TopicSortField,
};

#[derive(Debug, Clone)]
pub enum Action {
    // System
    Tick,
    Quit,
    Resize(u16, u16),

    // Navigation
    Navigate(Screen),
    GoBack,
    FocusSidebar,
    FocusContent,
    SelectSidebarItem(SidebarItem),

    // Connection
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

    // Topics
    FetchTopics,
    TopicsFetched(Vec<TopicInfo>),
    TopicsFetchFailed(String),
    SelectTopic(usize),
    FilterTopics(String),
    ClearTopicFilter,
    SortTopics(TopicSortField),
    CreateTopic { name: String, partitions: i32, replication_factor: i32 },
    TopicCreated { name: String, partitions: i32, replication_factor: i32 },
    TopicCreateFailed(String),
    DeleteTopic(String),
    TopicDeleted(String),
    TopicDeleteFailed(String),
    RequestViewTopicDetails,
    ViewTopicDetails(String),
    TopicDetailsFetched(TopicDetail),
    TopicDetailsFetchFailed(String),
    SwitchTopicDetailTab,
    ViewTopicMessages(String),

    // Topic Management
    AddPartitions { topic: String, new_count: i32 },
    PartitionsAdded(String),
    PartitionsAddFailed(String),
    AlterTopicConfig { topic: String, configs: Vec<(String, String)> },
    TopicConfigAltered(String),
    TopicConfigAlterFailed(String),
    PurgeTopic { topic: String, before_offset: i64 },
    TopicPurged(String),
    TopicPurgeFailed(String),
    UpdateAddPartitionsForm(AddPartitionsFormState),
    UpdateAlterConfigForm(AlterConfigFormState),
    UpdatePurgeTopicForm(PurgeTopicFormState),

    // Messages
    FetchMessages { topic: String, offset_mode: OffsetMode, partition: Option<i32> },
    MessagesFetched(Vec<KafkaMessage>),
    MessageReceived(KafkaMessage),
    MessagesFetchFailed(String),
    SelectMessage(usize),
    SetOffsetMode(OffsetMode),
    SetPartitionFilter(Option<i32>),
    StartConsuming { topic: String },
    StopConsuming,
    ProduceMessage { topic: String, key: Option<String>, value: String, headers: HashMap<String, String> },
    MessageProduced,
    MessageProduceFailed(String),
    ToggleMessageDetail,
    ClearMessages,

    // Consumer Groups
    FetchConsumerGroups,
    ConsumerGroupsFetched(Vec<ConsumerGroupInfo>),
    ConsumerGroupsFetchFailed(String),
    SelectConsumerGroup(usize),
    FilterConsumerGroups(String),
    ClearConsumerGroupFilter,
    ViewConsumerGroupDetails(String),
    ConsumerGroupDetailsFetched(ConsumerGroupDetail),
    ConsumerGroupDetailsFetchFailed(String),
    SwitchConsumerGroupDetailTab,

    // Brokers
    FetchBrokers,
    BrokersFetched { brokers: Vec<BrokerInfo>, cluster_id: Option<String> },
    BrokersFetchFailed(String),

    // UI
    ShowHelp,
    HideHelp,
    ShowModal(ModalType),
    HideModal,
    ModalConfirm,
    ModalCancel,
    UpdateModalInput(String),
    UpdateConnectionForm(ConnectionFormState),
    UpdateTopicCreateForm(TopicCreateFormState),
    UpdateProduceForm(ProduceFormState),
    ShowToast { message: String, level: ToastLevel },
    DismissToast(uuid::Uuid),

    // Navigation
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

#[derive(Debug)]
pub enum Command {
    None,
    Batch(Vec<Command>),

    // Kafka
    ConnectToKafka(ConnectionProfile),
    DisconnectFromKafka,
    FetchTopicList,
    FetchTopicDetails(String),
    CreateKafkaTopic { name: String, partitions: i32, replication_factor: i32 },
    DeleteKafkaTopic(String),
    FetchMessages { topic: String, offset_mode: OffsetMode, partition: Option<i32>, limit: usize },
    StartMessageConsumer { topic: String, offset_mode: OffsetMode, partition: Option<i32> },
    StopMessageConsumer,
    ProduceKafkaMessage { topic: String, key: Option<String>, value: String, headers: HashMap<String, String> },
    FetchConsumerGroupList,
    FetchConsumerGroupDetails(String),
    FetchBrokerList,

    // Topic Management
    AddTopicPartitions { topic: String, new_count: i32 },
    AlterKafkaTopicConfig { topic: String, configs: Vec<(String, String)> },
    PurgeKafkaTopic { topic: String, before_offset: i64 },

    // Storage
    LoadConnectionProfiles,
    SaveConnectionProfile(ConnectionProfile),
    DeleteConnectionProfile(uuid::Uuid),
}
