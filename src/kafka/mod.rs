use actix_web::web::block;
use once_cell::sync::OnceCell;
use producer::kafka_producer;
use rdkafka::producer::FutureProducer;
use serde::{Deserialize, Serialize};

use crate::models::RequestResult;

mod producer;
pub use producer::KafkaProducer;

enum KafkaTopic {
    REQUEST_EVENT,
}

impl KafkaTopic {
    fn as_str(&self) -> &'static str {
        match self {
            KafkaTopic::REQUEST_EVENT => "request_event",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestEvent {
    id: String,
    url: String,
    timestamp: String,
    response_time: i32,
}

impl RequestEvent {
    pub fn from(request_event: RequestResult) -> Self {
        return RequestEvent {
            id: request_event.id.to_string(),
            url: request_event.url,
            timestamp: format!("{}", request_event.timestamp),
            response_time: request_event.response_time,
        };
    }
}
