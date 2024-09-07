use std::env;

use ramlich::kafka::{self, KafkaConsumer};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let brokers = env::var("RUST_BROKERS").unwrap_or("127.0.0.1:29092".to_string());
    let consumer = KafkaConsumer::init(
        &brokers,
        &[kafka::KafkaTopic::RequestEvent.as_str().to_string()],
    );
    consumer.consume().await;
}
