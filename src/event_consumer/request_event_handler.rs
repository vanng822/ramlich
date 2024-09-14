use async_trait::async_trait;
use log::{error, info};

use crate::kafka::{KafkaTopic, RequestEvent, TopicHandler};

use super::db::add_request_event;

pub struct RequestEventHandler {}

impl RequestEventHandler {
    async fn handle_request_event(&self, payload: &str) {
        let request: RequestEvent = serde_json::from_str(payload).unwrap();
        let result = add_request_event(request).await;
        match result {
            Ok(stored_request) => info!("{:#?}", stored_request),
            Err(err) => error!("{:#?}", err),
        }
    }
}

#[async_trait]
impl TopicHandler for RequestEventHandler {
    async fn handle(&self, topic_name: &str, payload: &str) {
        match KafkaTopic::from_str(topic_name).unwrap() {
            KafkaTopic::RequestEvent => self.handle_request_event(payload).await,
        }
    }
}
