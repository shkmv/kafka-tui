use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext};
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{FutureProducer, FutureRecord, ProducerContext};
use rdkafka::TopicPartitionList;

use crate::app::state::{
    ConsumerGroupDetail, ConsumerGroupInfo, GroupMember, KafkaMessage, OffsetMode,
    PartitionInfo, PartitionOffset, TopicDetail, TopicInfo, TopicPartition,
};
use crate::error::{AppError, AppResult};
use crate::kafka::config::{KafkaConfig, KafkaSaslMechanism, SecurityConfig};

#[derive(Clone)]
struct SilentContext;

impl ClientContext for SilentContext {
    fn log(&self, _: RDKafkaLogLevel, _: &str, _: &str) {}
    fn error(&self, _: rdkafka::error::KafkaError, _: &str) {}
}
impl ConsumerContext for SilentContext {}
impl ProducerContext for SilentContext {
    type DeliveryOpaque = ();
    fn delivery(&self, _: &rdkafka::producer::DeliveryResult<'_>, _: ()) {}
}

pub struct KafkaClient {
    config: KafkaConfig,
    admin: AdminClient<SilentContext>,
    consumer: BaseConsumer<SilentContext>,
    producer: FutureProducer<SilentContext>,
}

impl KafkaClient {
    pub async fn new(config: KafkaConfig) -> AppResult<Arc<Self>> {
        let mut base = Self::base_config(&config);

        let admin = base.clone()
            .create_with_context(SilentContext)
            .map_err(|e| AppError::Kafka(format!("Admin client: {}", e)))?;

        let consumer = base.clone()
            .set("group.id", "kafka-tui-browser")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create_with_context(SilentContext)
            .map_err(|e| AppError::Kafka(format!("Consumer: {}", e)))?;

        let producer = base
            .set("message.timeout.ms", "5000")
            .create_with_context(SilentContext)
            .map_err(|e| AppError::Kafka(format!("Producer: {}", e)))?;

        Ok(Arc::new(Self { config, admin, consumer, producer }))
    }

    fn base_config(config: &KafkaConfig) -> ClientConfig {
        let mut c = ClientConfig::new();
        c.set("bootstrap.servers", &config.brokers)
            .set("socket.timeout.ms", config.connection_timeout_ms.to_string())
            .set("request.timeout.ms", config.request_timeout_ms.to_string())
            .set("socket.connection.setup.timeout.ms", "5000")
            .set("reconnect.backoff.ms", "100")
            .set("reconnect.backoff.max.ms", "1000");

        match &config.security {
            SecurityConfig::None => {}
            SecurityConfig::SaslPlain { username, password } => {
                c.set("security.protocol", "SASL_PLAINTEXT")
                    .set("sasl.mechanism", "PLAIN")
                    .set("sasl.username", username)
                    .set("sasl.password", password);
            }
            SecurityConfig::SaslScram256 { username, password } => {
                c.set("security.protocol", "SASL_PLAINTEXT")
                    .set("sasl.mechanism", "SCRAM-SHA-256")
                    .set("sasl.username", username)
                    .set("sasl.password", password);
            }
            SecurityConfig::SaslScram512 { username, password } => {
                c.set("security.protocol", "SASL_PLAINTEXT")
                    .set("sasl.mechanism", "SCRAM-SHA-512")
                    .set("sasl.username", username)
                    .set("sasl.password", password);
            }
            SecurityConfig::Ssl { ca_location, cert_location, key_location, key_password } => {
                c.set("security.protocol", "SSL");
                if let Some(v) = ca_location { c.set("ssl.ca.location", v); }
                if let Some(v) = cert_location { c.set("ssl.certificate.location", v); }
                if let Some(v) = key_location { c.set("ssl.key.location", v); }
                if let Some(v) = key_password { c.set("ssl.key.password", v); }
            }
            SecurityConfig::SaslSsl { mechanism, username, password, ca_location } => {
                let mech = match mechanism {
                    KafkaSaslMechanism::Plain => "PLAIN",
                    KafkaSaslMechanism::ScramSha256 => "SCRAM-SHA-256",
                    KafkaSaslMechanism::ScramSha512 => "SCRAM-SHA-512",
                };
                c.set("security.protocol", "SASL_SSL")
                    .set("sasl.mechanism", mech)
                    .set("sasl.username", username)
                    .set("sasl.password", password);
                if let Some(v) = ca_location { c.set("ssl.ca.location", v); }
            }
        }
        c
    }

    pub async fn test_connection(&self) -> AppResult<()> {
        self.consumer
            .fetch_metadata(None, Duration::from_secs(10))
            .map_err(|e| AppError::Kafka(format!("Connection failed: {}", e)))?;
        Ok(())
    }

    pub async fn list_topics(&self) -> AppResult<Vec<TopicInfo>> {
        let metadata = self.consumer
            .fetch_metadata(None, Duration::from_secs(30))
            .map_err(|e| AppError::Kafka(format!("Metadata fetch failed: {}", e)))?;

        let mut topics: Vec<_> = metadata.topics().iter().map(|t| {
            let partitions = t.partitions();
            TopicInfo {
                name: t.name().to_string(),
                partition_count: partitions.len() as i32,
                replication_factor: partitions.first().map(|p| p.replicas().len() as i32).unwrap_or(0),
                message_count: None,
                is_internal: t.name().starts_with("__"),
            }
        }).collect();

        topics.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(topics)
    }

    pub async fn create_topic(&self, name: &str, partitions: i32, replication: i32) -> AppResult<()> {
        let topic = NewTopic::new(name, partitions, TopicReplication::Fixed(replication));
        let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(30)));

        let results = self.admin.create_topics(&[topic], &opts).await
            .map_err(|e| AppError::Kafka(format!("Create topic failed: {}", e)))?;

        for r in results {
            if let Err((_, e)) = r {
                return Err(AppError::Kafka(format!("Create topic failed: {:?}", e)));
            }
        }
        Ok(())
    }

    pub async fn delete_topic(&self, name: &str) -> AppResult<()> {
        let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(30)));

        let results = self.admin.delete_topics(&[name], &opts).await
            .map_err(|e| AppError::Kafka(format!("Delete topic failed: {}", e)))?;

        for r in results {
            if let Err((_, e)) = r {
                return Err(AppError::Kafka(format!("Delete topic failed: {:?}", e)));
            }
        }
        Ok(())
    }

    pub async fn fetch_messages(
        &self,
        topic: &str,
        offset_mode: OffsetMode,
        partition: Option<i32>,
        limit: usize,
    ) -> AppResult<Vec<KafkaMessage>> {
        let metadata = self.consumer
            .fetch_metadata(Some(topic), Duration::from_secs(10))
            .map_err(|e| AppError::Kafka(format!("Topic metadata: {}", e)))?;

        let topic_meta = metadata.topics().first()
            .ok_or_else(|| AppError::Kafka("Topic not found".into()))?;

        let partitions: Vec<i32> = partition
            .map(|p| vec![p])
            .unwrap_or_else(|| topic_meta.partitions().iter().map(|p| p.id()).collect());

        let mut tpl = TopicPartitionList::new();
        for &p in &partitions {
            tpl.add_partition(topic, p);
            let offset = match &offset_mode {
                OffsetMode::Earliest => rdkafka::Offset::Beginning,
                OffsetMode::Specific(o) => rdkafka::Offset::Offset(*o),
                OffsetMode::Timestamp(ts) => rdkafka::Offset::Offset(ts.timestamp_millis()),
                OffsetMode::Latest => {
                    let (_, high) = self.consumer
                        .fetch_watermarks(topic, p, Duration::from_secs(10))
                        .map_err(|e| AppError::Kafka(format!("Watermarks: {}", e)))?;
                    rdkafka::Offset::Offset((high - limit as i64).max(0))
                }
            };
            tpl.set_partition_offset(topic, p, offset)
                .map_err(|e| AppError::Kafka(format!("Set offset: {}", e)))?;
        }

        self.consumer.assign(&tpl)
            .map_err(|e| AppError::Kafka(format!("Assign: {}", e)))?;

        let mut messages = Vec::with_capacity(limit);
        let deadline = std::time::Instant::now() + Duration::from_secs(5);

        while messages.len() < limit && std::time::Instant::now() < deadline {
            match self.consumer.poll(Duration::from_millis(100)) {
                Some(Ok(msg)) => messages.push(Self::parse_message(&msg)),
                Some(Err(_)) => {}
                None if messages.is_empty() => continue,
                None => break,
            }
        }

        self.consumer.unassign().ok();
        Ok(messages)
    }

    fn parse_message(msg: &rdkafka::message::BorrowedMessage<'_>) -> KafkaMessage {
        KafkaMessage {
            partition: msg.partition(),
            offset: msg.offset(),
            timestamp: msg.timestamp().to_millis()
                .and_then(chrono::DateTime::from_timestamp_millis),
            key: msg.key().map(|k| String::from_utf8_lossy(k).into()),
            value: msg.payload().map(|v| String::from_utf8_lossy(v).into()).unwrap_or_default(),
            headers: msg.headers().map(|h| {
                h.iter()
                    .filter_map(|hdr| hdr.value.map(|v| (hdr.key.into(), String::from_utf8_lossy(v).into())))
                    .collect()
            }).unwrap_or_default(),
        }
    }

    pub async fn produce_message(
        &self,
        topic: &str,
        key: Option<&str>,
        value: &str,
        headers: &HashMap<String, String>,
    ) -> AppResult<()> {
        let mut record: FutureRecord<'_, str, str> = FutureRecord::to(topic).payload(value);
        if let Some(k) = key {
            record = record.key(k);
        }

        let owned_headers = headers.iter().fold(
            rdkafka::message::OwnedHeaders::new(),
            |h, (k, v)| h.insert(rdkafka::message::Header { key: k, value: Some(v.as_bytes()) })
        );

        self.producer
            .send(record.headers(owned_headers), Duration::from_secs(5))
            .await
            .map_err(|(e, _)| AppError::Kafka(format!("Produce failed: {}", e)))?;
        Ok(())
    }

    pub async fn list_consumer_groups(&self) -> AppResult<Vec<ConsumerGroupInfo>> {
        let groups = self.admin.inner()
            .fetch_group_list(None, Duration::from_secs(30))
            .map_err(|e| AppError::Kafka(format!("Fetch groups: {}", e)))?;

        Ok(groups.groups().iter()
            .filter(|g| g.name() != "kafka-tui-browser")
            .map(|g| ConsumerGroupInfo {
                group_id: g.name().into(),
                state: g.state().into(),
                members_count: g.members().len(),
                topics: vec![],
                total_lag: 0,
            })
            .collect())
    }

    pub async fn get_topic_details(&self, topic_name: &str) -> AppResult<TopicDetail> {
        let metadata = self.consumer
            .fetch_metadata(Some(topic_name), Duration::from_secs(10))
            .map_err(|e| AppError::Kafka(format!("Metadata fetch: {}", e)))?;

        let topic_meta = metadata.topics().first()
            .ok_or_else(|| AppError::Kafka("Topic not found".into()))?;

        let mut partitions = Vec::new();
        for p in topic_meta.partitions() {
            let (low, high) = self.consumer
                .fetch_watermarks(topic_name, p.id(), Duration::from_secs(5))
                .unwrap_or((0, 0));

            partitions.push(PartitionInfo {
                id: p.id(),
                leader: p.leader(),
                replicas: p.replicas().to_vec(),
                isr: p.isr().to_vec(),
                low_watermark: low,
                high_watermark: high,
            });
        }

        partitions.sort_by_key(|p| p.id);

        // Fetch config using admin API
        let config = self.get_topic_config(topic_name).await.unwrap_or_default();

        Ok(TopicDetail {
            name: topic_name.to_string(),
            partitions,
            config,
            is_internal: topic_name.starts_with("__"),
        })
    }

    async fn get_topic_config(&self, topic_name: &str) -> AppResult<Vec<(String, String)>> {
        use rdkafka::admin::ResourceSpecifier;

        let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(10)));
        let resource = ResourceSpecifier::Topic(topic_name);

        let results = self.admin.describe_configs([&resource], &opts).await
            .map_err(|e| AppError::Kafka(format!("Describe config: {}", e)))?;

        let mut config = Vec::new();
        for result in results {
            match result {
                Ok(resource) => {
                    for entry in resource.entries {
                        if let Some(value) = entry.value {
                            config.push((entry.name, value));
                        }
                    }
                }
                Err(e) => return Err(AppError::Kafka(format!("Config error: {:?}", e))),
            }
        }

        config.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(config)
    }

    pub async fn get_consumer_group_details(&self, group_id: &str) -> AppResult<ConsumerGroupDetail> {
        // Get group description - extract data before any await
        let (state, members) = {
            let groups = self.admin.inner()
                .fetch_group_list(Some(group_id), Duration::from_secs(10))
                .map_err(|e| AppError::Kafka(format!("Fetch group: {}", e)))?;

            let group = groups.groups().iter()
                .find(|g| g.name() == group_id)
                .ok_or_else(|| AppError::Kafka("Group not found".into()))?;

            let state = group.state().to_string();
            let members: Vec<GroupMember> = group.members().iter().map(|m| {
                GroupMember {
                    member_id: m.id().to_string(),
                    client_id: m.client_id().to_string(),
                    client_host: m.client_host().to_string(),
                    assignments: Self::parse_member_assignment(m.assignment().unwrap_or(&[])),
                }
            }).collect();

            (state, members)
        };

        // Get committed offsets for the group
        let offsets = self.get_group_offsets(group_id).await.unwrap_or_default();

        Ok(ConsumerGroupDetail {
            group_id: group_id.to_string(),
            state,
            coordinator: None,
            members,
            offsets,
        })
    }

    fn parse_member_assignment(data: &[u8]) -> Vec<TopicPartition> {
        // Member assignment is binary protocol, simplified parsing
        // Format: version(2) + topic_count(4) + [topic_name_len(2) + topic_name + partition_count(4) + [partition(4)]]
        if data.len() < 6 {
            return vec![];
        }

        let mut result = Vec::new();
        let mut pos = 2; // skip version

        if pos + 4 > data.len() { return result; }
        let topic_count = i32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        pos += 4;

        for _ in 0..topic_count {
            if pos + 2 > data.len() { break; }
            let topic_len = i16::from_be_bytes([data[pos], data[pos+1]]) as usize;
            pos += 2;

            if pos + topic_len > data.len() { break; }
            let topic = String::from_utf8_lossy(&data[pos..pos+topic_len]).to_string();
            pos += topic_len;

            if pos + 4 > data.len() { break; }
            let partition_count = i32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
            pos += 4;

            for _ in 0..partition_count {
                if pos + 4 > data.len() { break; }
                let partition = i32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]);
                pos += 4;
                result.push(TopicPartition { topic: topic.clone(), partition });
            }
        }

        result
    }

    async fn get_group_offsets(&self, group_id: &str) -> AppResult<Vec<PartitionOffset>> {
        // Create a temporary consumer for this group to fetch offsets
        let mut config = Self::base_config(&self.config);
        config.set("group.id", group_id);

        let consumer: BaseConsumer<SilentContext> = config
            .create_with_context(SilentContext)
            .map_err(|e| AppError::Kafka(format!("Consumer for offsets: {}", e)))?;

        let committed = consumer
            .committed(Duration::from_secs(10))
            .map_err(|e| AppError::Kafka(format!("Fetch committed: {}", e)))?;

        let mut offsets = Vec::new();
        for elem in committed.elements() {
            let current_offset = match elem.offset() {
                rdkafka::Offset::Offset(o) => o,
                _ => continue,
            };

            // Get log end offset (high watermark)
            let (_, high) = self.consumer
                .fetch_watermarks(elem.topic(), elem.partition(), Duration::from_secs(5))
                .unwrap_or((0, 0));

            offsets.push(PartitionOffset {
                topic: elem.topic().to_string(),
                partition: elem.partition(),
                current_offset,
                log_end_offset: high,
                lag: (high - current_offset).max(0),
            });
        }

        offsets.sort_by(|a, b| (&a.topic, a.partition).cmp(&(&b.topic, b.partition)));
        Ok(offsets)
    }

    pub fn brokers(&self) -> &str {
        &self.config.brokers
    }
}
