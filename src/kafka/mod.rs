use serde::{Deserialize, Serialize};

use crate::models::RequestResult;

mod producer;
pub use producer::KafkaProducer;

mod consumer;
pub use consumer::KafkaConsumer;

pub enum KafkaTopic {
    RequestEvent,
}

impl KafkaTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            KafkaTopic::RequestEvent => "request_event",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestEvent {
    id: String,
    url: String,
    timestamp: i64,
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
