use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event;
use ratatui::prelude::*;
use tokio::sync::mpsc;

use crate::app::actions::{Action, Command};
use crate::app::state::{AppState, ToastLevel};
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

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { state: AppState::default(), tx, rx, client: None }
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
                            self.tx.send(Action::ConnectionSuccess).ok();
                        }
                        Err(e) => { self.tx.send(Action::ConnectionFailed(e.to_string())).ok(); }
                    },
                    Err(e) => { self.tx.send(Action::ConnectionFailed(e.to_string())).ok(); }
                }
            }

            Command::DisconnectFromKafka => {
                self.client = None;
            }

            Command::FetchTopicList => {
                self.spawn_kafka(|c, tx| async move {
                    match c.list_topics().await {
                        Ok(t) => { tx.send(Action::TopicsFetched(t)).ok(); }
                        Err(e) => { tx.send(Action::TopicsFetchFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::FetchTopicDetails(name) => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.get_topic_details(&name).await {
                        Ok(d) => { tx.send(Action::TopicDetailsFetched(d)).ok(); }
                        Err(e) => { tx.send(Action::TopicDetailsFetchFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::CreateKafkaTopic { name, partitions, replication_factor } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.create_topic(&name, partitions, replication_factor).await {
                        Ok(_) => { tx.send(Action::TopicCreated { name, partitions, replication_factor }).ok(); }
                        Err(e) => { tx.send(Action::TopicCreateFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::DeleteKafkaTopic(name) => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.delete_topic(&name).await {
                        Ok(_) => { tx.send(Action::TopicDeleted(name)).ok(); }
                        Err(e) => { tx.send(Action::TopicDeleteFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::FetchMessages { topic, offset_mode, partition, limit } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.fetch_messages(&topic, offset_mode, partition, limit).await {
                        Ok(m) => { tx.send(Action::MessagesFetched(m)).ok(); }
                        Err(e) => { tx.send(Action::MessagesFetchFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::StartMessageConsumer { .. } | Command::StopMessageConsumer => {}

            Command::ProduceKafkaMessage { topic, key, value, headers } => {
                self.spawn_kafka(move |c, tx| async move {
                    match c.produce_message(&topic, key.as_deref(), &value, &headers).await {
                        Ok(_) => { tx.send(Action::MessageProduced).ok(); }
                        Err(e) => { tx.send(Action::MessageProduceFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::FetchConsumerGroupList => {
                self.spawn_kafka(|c, tx| async move {
                    match c.list_consumer_groups().await {
                        Ok(g) => { tx.send(Action::ConsumerGroupsFetched(g)).ok(); }
                        Err(e) => { tx.send(Action::ConsumerGroupsFetchFailed(e.to_string())).ok(); }
                    }
                });
            }

            Command::FetchConsumerGroupDetails(_) => {}

            Command::LoadConnectionProfiles => {
                match connections::load_connections() {
                    Ok(p) => { self.tx.send(Action::ConnectionsLoaded(p)).ok(); }
                    Err(e) => {
                        self.tx.send(Action::ShowToast { message: e.to_string(), level: ToastLevel::Error }).ok();
                        self.tx.send(Action::ConnectionsLoaded(vec![])).ok();
                    }
                }
            }

            Command::SaveConnectionProfile(p) => {
                if let Err(e) = connections::save_connection(&p) {
                    self.tx.send(Action::ShowToast { message: e.to_string(), level: ToastLevel::Error }).ok();
                }
            }

            Command::DeleteConnectionProfile(id) => {
                match connections::delete_connection(id) {
                    Ok(_) => { self.tx.send(Action::ConnectionDeleted(id)).ok(); }
                    Err(e) => { self.tx.send(Action::ShowToast { message: e.to_string(), level: ToastLevel::Error }).ok(); }
                }
            }

            Command::SaveToHistory(_) | Command::LoadHistory | Command::ScheduleTick(_) => {}
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
                self.tx.send(Action::ShowToast {
                    message: "Not connected to Kafka".into(),
                    level: ToastLevel::Error,
                }).ok();
            }
        }
    }
}

impl Default for App {
    fn default() -> Self { Self::new() }
}
