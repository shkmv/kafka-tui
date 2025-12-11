use rdkafka::admin::AdminClient;
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext};
use rdkafka::producer::{FutureProducer, ProducerContext};
use std::time::Duration;

#[derive(Clone)]
struct SilentContext;

impl ClientContext for SilentContext {
    fn log(&self, _level: RDKafkaLogLevel, _fac: &str, _log_message: &str) {}
    fn error(&self, _error: rdkafka::error::KafkaError, _reason: &str) {}
}

impl ConsumerContext for SilentContext {}

impl ProducerContext for SilentContext {
    type DeliveryOpaque = ();
    fn delivery(
        &self,
        _delivery_result: &rdkafka::producer::DeliveryResult<'_>,
        _delivery_opaque: Self::DeliveryOpaque,
    ) {
    }
}

fn main() {
    println!("Testing Kafka connection with SilentContext...\n");

    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", "localhost:9092")
        .set("socket.timeout.ms", "10000")
        .set("request.timeout.ms", "15000")
        .set("socket.connection.setup.timeout.ms", "5000")
        .set("reconnect.backoff.ms", "100")
        .set("reconnect.backoff.max.ms", "1000");

    // Test AdminClient
    println!("1. Creating AdminClient...");
    let admin: Result<AdminClient<SilentContext>, _> = config
        .clone()
        .create_with_context(SilentContext);
    match admin {
        Ok(_) => println!("   ✓ AdminClient created successfully"),
        Err(e) => println!("   ✗ AdminClient error: {:?}", e),
    }

    // Test Consumer
    println!("\n2. Creating BaseConsumer...");
    let consumer: Result<BaseConsumer<SilentContext>, _> = config
        .clone()
        .set("group.id", "kafka-tui-test")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create_with_context(SilentContext);
    match &consumer {
        Ok(_) => println!("   ✓ BaseConsumer created successfully"),
        Err(e) => println!("   ✗ BaseConsumer error: {:?}", e),
    }

    // Test fetch metadata
    if let Ok(c) = &consumer {
        println!("\n3. Fetching metadata...");
        match c.fetch_metadata(None, Duration::from_secs(10)) {
            Ok(metadata) => {
                println!("   ✓ Metadata fetched!");
                println!("   Brokers:");
                for broker in metadata.brokers() {
                    println!("     - {}:{}", broker.host(), broker.port());
                }
                println!("   Topics: {}", metadata.topics().len());
            }
            Err(e) => println!("   ✗ Metadata error: {:?}", e),
        }
    }

    // Test Producer
    println!("\n4. Creating FutureProducer...");
    let producer: Result<FutureProducer<SilentContext>, _> = config
        .clone()
        .set("message.timeout.ms", "5000")
        .create_with_context(SilentContext);
    match producer {
        Ok(_) => println!("   ✓ FutureProducer created successfully"),
        Err(e) => println!("   ✗ FutureProducer error: {:?}", e),
    }

    println!("\nAll tests completed!");
}
