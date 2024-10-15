use std::env;

use actix_web::{middleware, web, App, HttpServer};
use log::info;
use ramlich::event_consumer::routes::get_request_event_by_id;
use ramlich::event_consumer::run_consumer;
use ramlich::postres::DBPool;
use ramlich::unleash::{init_client, sync_features};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let unleash_api_url =
        env::var("RUST_UNLEASH_API_URL").unwrap_or("http://127.0.0.1:4242/api/".to_string());
    let unleash_authorization = Some(
        env::var("RUST_UNLEASH_AUTHORIZATION")
            .unwrap_or("default:development.unleash-insecure-api-token".to_string()),
    );

    init_client("event_consumer", &unleash_api_url, unleash_authorization).await;

    let db_port: u16 = env::var("RUST_DB_PORT")
        .unwrap_or("5532".to_string())
        .parse()
        .unwrap();
    let db_host = env::var("RUST_DB_HOST").unwrap_or("localhost".to_string());
    let db_user = env::var("RUST_DB_USER").unwrap_or("ramlich".to_string());
    let db_dbname = env::var("RUST_DB_DBNAME").unwrap_or("ramlich".to_string());

    // TODO fix postres docker
    DBPool::init(db_port, db_host, db_user, db_dbname);

    let brokers = env::var("RUST_BROKERS").unwrap_or("127.0.0.1:29092".to_string());
    info!("brokers: {}", brokers);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async move {
        sync_features().await;
    });

    rt.spawn(run_consumer(brokers));

    let port = env::var("RUST_PORT").unwrap_or("8585".to_string());
    let host = env::var("RUST_HOST").unwrap_or("127.0.0.1".to_string());
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_request_event_by_id)
            .service(web::resource("/healthcheck").to(|| async { "OK" }))
    })
    .bind(format!("{}:{}", host, port))
    .unwrap()
    .run()
    .await;
}
