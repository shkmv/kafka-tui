use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event;
use ratatui::prelude::*;
use tokio::sync::mpsc;

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, Level};
use crate::app::update::update;
use crate::events::handler::EventHandler;
use crate::kafka::config::KafkaConfig;
use crate::kafka::KafkaClient;
use crate::storage::connections;
use crate::ui::render::render_app;

pub struct App {
    state: AppState,
    tx: mpsc::UnboundedSender<Action>,
    rx: mpsc::UnboundedReceiver<Action>,
    client: Option<Arc<KafkaClient>>,
}

/// Helper function to send an action and log if the channel is closed.
fn send_action(tx: &mpsc::UnboundedSender<Action>, action: Action) {
    if tx.send(action).is_err() {
        tracing::warn!("Channel send failed - receiver dropped");
    }
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { state: AppState::default(), tx, rx, client: None }
    }

    /// Send an action to the channel, logging if the send fails.
    fn send(&self, action: Action) {
        send_action(&self.tx, action);
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        self.state.running = true;
        self.exec(Command::LoadConnectionProfiles).await;

        while self.state.running {
            terminal.draw(|f| render_app(f, &self.state))?;

            let cmd = if event::poll(Duration::from_millis(100))? {
                EventHandler::handle_event(event::read()?, &self.state)
                    .map(|a| update(&mut self.state, a))
                    .unwrap_or(Command::None)
            } else {
                update(&mut self.state, Action::Tick)
            };
            self.exec(cmd).await;

            while let Ok(action) = self.rx.try_recv() {
                let cmd = update(&mut self.state, action);
                self.exec(cmd).await;
            }
        }
        Ok(())
    }

    async fn exec(&mut self, cmd: Command) {
        match cmd {
            Command::None => {}
            Command::Batch(cmds) => {
                for c in cmds {
                    Box::pin(self.exec(c)).await;
                }
            }

            Command::ConnectToKafka(profile) => {
                let config = KafkaConfig::from(profile);
                match KafkaClient::new(config).await {
                    Ok(c) => match c.test_connection().await {
                        Ok(_) => {
                            self.client = Some(c);
                            self.send(Action::ConnectionSuccess);
                        }
                        Err(e) => { self.send(Action::ConnectionFailed(e.to_string())); }
                    },
                    Err(e) => { self.send(Action::ConnectionFailed(e.to_string())); }
                }
            }

            Command::DisconnectFromKafka => {
                self.client = None;
            }

            Command::FetchTopicList => {
                self.spawn_kafka(|c, tx| async move {
                    match c.list_topics().await {
                        Ok(t) => send_action(&tx, Action::TopicsFetched(t)),
                        Err(e) => send_action(&tx, Action::TopicsFetchFailed(e.to_string())),
                    }
                });
            }

            Command::FetchTopicDetails(name) => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.get_topic_details(&name).await {
                        Ok(d) => send_action(&tx, Action::TopicDetailsFetched(d)),
                        Err(e) => send_action(&tx, Action::TopicDetailsFetchFailed(e.to_string())),
                    }
                });
            }

            Command::CreateKafkaTopic { name, partitions, replication_factor } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.create_topic(&name, partitions, replication_factor).await {
                        Ok(_) => send_action(&tx, Action::TopicCreated { name, partitions, replication_factor }),
                        Err(e) => send_action(&tx, Action::TopicCreateFailed(e.to_string())),
                    }
                });
            }

            Command::DeleteKafkaTopic(name) => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.delete_topic(&name).await {
                        Ok(_) => send_action(&tx, Action::TopicDeleted(name)),
                        Err(e) => send_action(&tx, Action::TopicDeleteFailed(e.to_string())),
                    }
                });
            }

            Command::FetchMessages { topic, offset_mode, partition, limit } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.fetch_messages(&topic, offset_mode, partition, limit).await {
                        Ok(m) => send_action(&tx, Action::MessagesFetched(m)),
                        Err(e) => send_action(&tx, Action::MessagesFetchFailed(e.to_string())),
                    }
                });
            }

            Command::StartMessageConsumer { .. } | Command::StopMessageConsumer => {}

            Command::ProduceKafkaMessage { topic, key, value, headers } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.produce_message(&topic, key.as_deref(), &value, &headers).await {
                        Ok(_) => send_action(&tx, Action::MessageProduced),
                        Err(e) => send_action(&tx, Action::MessageProduceFailed(e.to_string())),
                    }
                });
            }

            Command::FetchConsumerGroupList => {
                self.spawn_kafka(|c, tx| async move {
                    match c.list_consumer_groups().await {
                        Ok(g) => send_action(&tx, Action::ConsumerGroupsFetched(g)),
                        Err(e) => send_action(&tx, Action::ConsumerGroupsFetchFailed(e.to_string())),
                    }
                });
            }

            Command::FetchConsumerGroupDetails(group_id) => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.get_consumer_group_details(&group_id).await {
                        Ok(d) => send_action(&tx, Action::ConsumerGroupDetailsFetched(d)),
                        Err(e) => send_action(&tx, Action::ConsumerGroupDetailsFetchFailed(e.to_string())),
                    }
                });
            }

            Command::FetchBrokerList => {
                self.spawn_kafka(|c, tx| async move {
                    match c.list_brokers().await {
                        Ok((brokers, cluster_id)) => send_action(&tx, Action::BrokersFetched { brokers, cluster_id }),
                        Err(e) => send_action(&tx, Action::BrokersFetchFailed(e.to_string())),
                    }
                });
            }

            Command::LoadConnectionProfiles => {
                match connections::load_connections() {
                    Ok(p) => self.send(Action::ConnectionsLoaded(p)),
                    Err(e) => {
                        self.send(Action::ShowToast { message: e.to_string(), level: Level::Error });
                        self.send(Action::ConnectionsLoaded(vec![]));
                    }
                }
            }

            Command::SaveConnectionProfile(p) => {
                if let Err(e) = connections::save_connection(&p) {
                    self.send(Action::ShowToast { message: e.to_string(), level: Level::Error });
                }
            }

            Command::DeleteConnectionProfile(id) => {
                match connections::delete_connection(id) {
                    Ok(_) => self.send(Action::ConnectionDeleted(id)),
                    Err(e) => self.send(Action::ShowToast { message: e.to_string(), level: Level::Error }),
                }
            }

            Command::AddTopicPartitions { topic, new_count } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.add_partitions(&topic, new_count).await {
                        Ok(_) => send_action(&tx, Action::PartitionsAdded(topic)),
                        Err(e) => send_action(&tx, Action::PartitionsAddFailed(e.to_string())),
                    }
                });
            }

            Command::AlterKafkaTopicConfig { topic, configs } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.alter_topic_config(&topic, &configs).await {
                        Ok(_) => send_action(&tx, Action::TopicConfigAltered(topic)),
                        Err(e) => send_action(&tx, Action::TopicConfigAlterFailed(e.to_string())),
                    }
                });
            }

            Command::PurgeKafkaTopic { topic, before_offset } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.delete_records(&topic, before_offset).await {
                        Ok(_) => send_action(&tx, Action::TopicPurged(topic)),
                        Err(e) => send_action(&tx, Action::TopicPurgeFailed(e.to_string())),
                    }
                });
            }
        }
    }

    fn spawn_kafka<F, Fut>(&self, f: F)
    where
        F: FnOnce(Arc<KafkaClient>, mpsc::UnboundedSender<Action>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        match &self.client {
            Some(c) => {
                let client = c.clone();
                let tx = self.tx.clone();
                tokio::spawn(async move { f(client, tx).await });
            }
            None => {
                self.send(Action::ShowToast {
                    message: "Not connected to Kafka".into(),
                    level: Level::Error,
                });
            }
        }
    }
}

impl Default for App {
    fn default() -> Self { Self::new() }
}
