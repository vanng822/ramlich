use ramlich::kafka::{self, KafkaConsumer};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let consumer = KafkaConsumer::init(
        "localhost:29092",
        &[kafka::KafkaTopic::RequestEvent.as_str().to_string()],
    );
    consumer.consume().await;
}
