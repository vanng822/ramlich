use std::borrow::Borrow;

use actix_web::cookie::time::Duration;
use log::{error, info, warn};
use once_cell::sync::OnceCell;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    ClientConfig, Message,
};

fn new_consumer(brokers: &str, topics: &[String]) -> Result<StreamConsumer, KafkaError> {
    let msg = topics.join(" ");
    let stream_consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "latest")
        .set("enable.partition.eof", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()?;
    let topics = topics
        .iter()
        .map(|topic| topic.as_str())
        .collect::<Vec<&str>>();
    stream_consumer.subscribe(topics.as_slice())?;
    Ok(stream_consumer)
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

static INSTANCE: OnceCell<KafkaConsumer> = OnceCell::new();

impl KafkaConsumer {
    fn new(consumer: StreamConsumer) -> Self {
        return Self { consumer };
    }
    pub async fn consume(&self) {
        loop {
            match self.consumer.recv().await {
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    let topic_name = m.topic();
                    info!("topic_name: {}, payload: {}", topic_name, payload);
                }
                Err(kafka_error) => {
                    error!("{:#?}", kafka_error);
                }
            }
        }
    }

    pub fn instance() -> &'static KafkaConsumer {
        return INSTANCE.get().unwrap();
    }

    pub fn init(brokers: &str, topics: &[String]) -> &'static KafkaConsumer {
        let existing = INSTANCE.get();
        if existing.is_some() {
            return existing.unwrap();
        }

        let consumer = new_consumer(brokers, topics).unwrap();
        let kafka_consumer = Self::new(consumer);
        let _ = INSTANCE.set(kafka_consumer);

        return Self::instance();
    }
}
