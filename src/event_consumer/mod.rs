
use crate::kafka::{KafkaConsumer, TopicHandler};

pub mod db;
pub mod errors;
pub mod models;
pub mod request_event_handler;
pub mod routes;

pub fn run_consumer(brokers: String) -> tokio::task::JoinHandle<()> {
    actix_web::rt::spawn(async move {
        let handler: &dyn TopicHandler =
            &request_event_handler::RequestEventHandler {} as &dyn TopicHandler;
        let handlers: Vec<&'static dyn TopicHandler> = vec![handler];

        KafkaConsumer::new(&brokers.as_str(), handlers)
            .consume()
            .await;
    })
}
