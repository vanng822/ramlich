use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

use uuid::Uuid;

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
    pub id: Uuid,
    pub url: String,
    pub requested_at: DateTime<Utc>,
    pub response_time: i64,
}
