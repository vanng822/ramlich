use std::collections::HashMap;
use std::env;

use log::info;
use ramlich::event_consumer;
use ramlich::kafka::{self, KafkaConsumer, TopicHandler};
use ramlich::postres::DBPool;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let db_port: u16 = env::var("RUST_DB_PORT")
        .unwrap_or("5432".to_string())
        .parse()
        .unwrap();
    let db_host = env::var("RUST_DB_HOST").unwrap_or("localhost".to_string());
    let db_user = env::var("RUST_DB_USER").unwrap_or("ramlich".to_string());
    let db_dbname = env::var("RUST_DB_DBNAME").unwrap_or("ramlich".to_string());

    // TODO fix postres docker
    DBPool::init(db_port, db_host, db_user, db_dbname);

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
