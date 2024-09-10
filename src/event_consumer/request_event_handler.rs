use async_trait::async_trait;
use log::info;

use crate::kafka::TopicHandler;

pub struct RequestEventHandler {}

#[async_trait]
impl TopicHandler for RequestEventHandler {
    async fn handle(&self, topic_name: &str, payload: &str) {
        info!(
            "RequestEventHandler topic: {}, payload: {}",
            topic_name, payload
        );
    }
}
