use std::collections::HashMap;
use std::env;

use actix_web::{middleware, App, HttpServer};
use log::info;
use ramlich::event_consumer;
use ramlich::event_consumer::routes::get_request_event_by_id;
use ramlich::kafka::{self, KafkaConsumer, TopicHandler};
use ramlich::postres::DBPool;
use ramlich::unleash::{init_client, sync_features};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    init_client("event_consumer").await;

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

    actix_web::rt::spawn(async move {
        sync_features().await;
    });

    actix_web::rt::spawn(async move {
        let handler: &dyn TopicHandler =
            &event_consumer::request_event_handler::RequestEventHandler {} as &dyn TopicHandler;
        let handlers: HashMap<String, &'static dyn TopicHandler> = HashMap::from([(
            kafka::KafkaTopic::RequestEvent.as_str().to_string(),
            handler,
        )]);

        KafkaConsumer::new(&brokers, handlers).consume().await;
    });

    let port = env::var("RUST_PORT").unwrap_or("8585".to_string());
    let host = env::var("RUST_HOST").unwrap_or("127.0.0.1".to_string());
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_request_event_by_id)
    })
    .bind(format!("{}:{}", host, port))
    .unwrap()
    .workers(1)
    .run()
    .await;
}
