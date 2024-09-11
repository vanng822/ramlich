use async_trait::async_trait;

use crate::kafka::TopicHandler;

use super::{db::add_request_event, models::Request};

pub struct RequestEventHandler {}

#[async_trait]
impl TopicHandler for RequestEventHandler {
    async fn handle(&self, topic_name: &str, payload: &str) {
        let request: Request = serde_json::from_str(payload).unwrap();
        let _ = add_request_event(request).await;
    }
}
