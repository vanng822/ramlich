use std::thread;

use async_trait::async_trait;
use log::{error, info};

use crate::kafka::{KafkaTopic, RequestEvent, TopicHandler};

use super::db::add_request_event;

pub struct RequestEventHandler {}

impl RequestEventHandler {
    async fn handle_request_event(&self, payload: &str) {
        info!(
            "handle_request_event thread id: {:?}",
            thread::current().id()
        );
        let request: RequestEvent = serde_json::from_str(payload).unwrap();
        let result = add_request_event(request).await;
        match result {
            Ok(stored_id) => info!("stored_id: {}", stored_id),
            Err(err) => error!("{:#?}", err),
        }
    }
}

#[async_trait]
impl TopicHandler for RequestEventHandler {
    fn get_topic_name(&self) -> &str {
        KafkaTopic::RequestEvent.as_str()
    }
    async fn handle(&self, payload: &str) {
        info!("handle thread id: {:?}", thread::current().id());
        self.handle_request_event(payload).await;
    }
}
