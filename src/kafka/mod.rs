use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;
use strum::EnumString;

use crate::models::RequestResult;

mod producer;
pub use producer::KafkaProducer;

mod consumer;
pub use consumer::KafkaConsumer;
pub use consumer::TopicHandler;

pub enum KafkaTopic {
    RequestEvent,
}

impl KafkaTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            KafkaTopic::RequestEvent => "ramlich.request_event",
        }
    }
    pub fn from_str(topic_name: &str) -> Result<KafkaTopic, Box<dyn Error>> {
        match topic_name {
            "ramlich.request_event" => Ok(KafkaTopic::RequestEvent),
            _ => Err("VariantNotFound".into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestEvent {
    id: String,
    url: String,
    timestamp: SystemTime,
    response_time: i32,
}

impl RequestEvent {
    pub fn from(request_event: RequestResult) -> Self {
        return RequestEvent {
            id: request_event.id.to_string(),
            url: request_event.url,
            timestamp: request_event.timestamp,
            response_time: request_event.response_time,
        };
    }
}
