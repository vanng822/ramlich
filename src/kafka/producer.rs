use std::time::Duration;

use log::{error, info};
use once_cell::sync::OnceCell;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};

use super::KafkaTopic;

use super::RequestEvent;

fn kafka_producer(brokers: &str) -> Result<FutureProducer, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
}

pub struct KafkaProducer {
    producer: FutureProducer,
}

static INSTANCE: OnceCell<KafkaProducer> = OnceCell::new();

impl KafkaProducer {
    #[inline]
    pub fn get_producer(&self) -> FutureProducer {
        self.producer.clone()
    }

    pub async fn publish_request_event(&self, message: &RequestEvent) -> Option<KafkaError> {
        let payload = serde_json::to_string(&message).ok()?;
        info!("payload: {}", payload);
        let rec = FutureRecord::to(KafkaTopic::RequestEvent.as_str())
            .payload(&payload)
            .key("");
        let res = self.get_producer().send(rec, Duration::from_secs(0)).await;
        match res {
            Ok((_, _)) => {
                info!("publish successful");
                None
            }
            Err((err, _)) => {
                error!("publish failed: {}", err);
                Some(err)
            }
        }
    }

    pub fn instance() -> &'static KafkaProducer {
        INSTANCE.get().expect("KafkaProducer instance")
    }

    pub fn init(brokers: &str) -> &'static KafkaProducer {
        let existing = INSTANCE.get();
        if existing.is_some() {
            return existing.expect("KafkaProducer instance");
        }

        let producer: FutureProducer = kafka_producer(brokers).expect("FutureProducer created");
        let kafka_producer = Self { producer };
        let _ = INSTANCE.set(kafka_producer);

        Self::instance()
    }
}
