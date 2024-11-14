use std::collections::HashMap;

use async_trait::async_trait;
use log::{error, info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    ClientConfig, Message,
};

fn new_consumer(brokers: &str, topics: Vec<&str>) -> Result<StreamConsumer, KafkaError> {
    let stream_consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "latest")
        .set("enable.partition.eof", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create().expect("Failed to create new stram consumer");

    stream_consumer.subscribe(topics.as_slice()).expect("Failed to subscribe to the topic");
    Ok(stream_consumer)
}

#[async_trait]
pub trait TopicHandler {
    fn get_topic_name(&self) -> &str;
    async fn handle(&self, payload: &str);
}

pub struct KafkaConsumer<'a> {
    consumer: StreamConsumer,
    handler_mappings: HashMap<&'a str, &'a dyn TopicHandler>,
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
                    let handler = self.handler_mappings.get(topic_name);
                    if handler.is_some() {
                        // TODO: how to handle error in the best way
                        handler.unwrap().handle(payload).await;
                    } else {
                        warn!("Received message for topic: {topic_name} with no handler, payload: {payload}");
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
        handlers: Vec<&'static dyn TopicHandler>,
    ) -> KafkaConsumer<'a> {
        let mut topics: Vec<&str> = vec![];
        let mut handler_mappings: HashMap<&str, &'static dyn TopicHandler> = HashMap::new();

        for handler in handlers.iter() {
            let topic_name = handler.get_topic_name();
            if topics.contains(&topic_name) {
                panic!("can only have one handler per topic, found multiple handlers for: {topic_name}");
            }
            topics.push(topic_name);
            handler_mappings.insert(topic_name, handler.to_owned());
        }

        info!("topics: {:#?}", topics);
        let consumer = new_consumer(brokers, topics).expect("StreamConsumer created");
        Self {
            consumer,
            handler_mappings,
        }
    }
}
