use std::collections::HashMap;
use std::env;

use log::info;
use ramlich::event_consumer;
use ramlich::kafka::{self, KafkaConsumer, TopicHandler};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let brokers = env::var("RUST_BROKERS").unwrap_or("127.0.0.1:29092".to_string());
    info!("brokers: {}", brokers);

    let handler: &dyn TopicHandler =
        &event_consumer::request_event_handler::RequestEventHandler {} as &dyn TopicHandler;
    let handlers: HashMap<String, &'static dyn TopicHandler> = HashMap::from([(
        kafka::KafkaTopic::RequestEvent.as_str().to_string(),
        handler,
    )]);
    let consumer = KafkaConsumer::new(&brokers, handlers);
    consumer.consume().await;
}
