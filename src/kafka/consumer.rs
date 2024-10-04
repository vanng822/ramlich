use std::collections::HashMap;

use async_trait::async_trait;
use log::{error, info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    ClientConfig, Message,
};

fn new_consumer(brokers: &str, topics: &[String]) -> Result<StreamConsumer, KafkaError> {
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

#[async_trait]
pub trait TopicHandler {
    async fn handle(&self, topic_name: &str, payload: &str);
}

pub struct KafkaConsumer<'a> {
    consumer: StreamConsumer,
    handlers: HashMap<String, &'a dyn TopicHandler>,
}

impl KafkaConsumer<'static> {
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
                    let handle_fn = self.handlers.get(topic_name);
                    if handle_fn.is_some() {
                        // TODO: how to handle error in the best way
                        handle_fn.unwrap().handle(topic_name, payload).await;
                    }
                }
                Err(kafka_error) => {
                    error!("{:#?}", kafka_error);
                }
            }
        }
    }

    pub fn new<'a>(
        brokers: &str,
        handlers: HashMap<String, &'static dyn TopicHandler>,
    ) -> KafkaConsumer<'a> {
        let mut topics: Vec<String> = vec![];

        for (topic_name, _) in handlers.iter() {
            topics.push(topic_name.to_string());
        }

        info!("topics: {:#?}", topics);
        let consumer = new_consumer(brokers, &topics).expect("StreamConsumer created");
        let kafka_consumer = Self {
            consumer: consumer,
            handlers: handlers,
        };

        return kafka_consumer;
    }
}
