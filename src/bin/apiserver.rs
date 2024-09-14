use std::env;

use actix_web::{middleware, App, HttpServer};
use log::info;
use ramlich::handlers::{get_month_route, lunar_route, today_route, ApiDoc};
use ramlich::kafka::KafkaProducer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = env::var("RUST_PORT").unwrap_or("8181".to_string());
    let host = env::var("RUST_HOST").unwrap_or("127.0.0.1".to_string());
    let brokers = env::var("RUST_BROKERS").unwrap_or("127.0.0.1:29092".to_string());
    info!("brokers: {}", brokers);

    KafkaProducer::init(&brokers);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(today_route)
            .service(lunar_route)
            .service(get_month_route)
    })
    .bind(format!("{}:{}", host, port))?
    .workers(2)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_today_get() {
        KafkaProducer::init("localhost:29092");

        let app = test::init_service(App::new().service(today_route)).await;
        let req = test::TestRequest::get().uri("/today").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_lunar_get() {
        let app = test::init_service(App::new().service(lunar_route)).await;
        let req = test::TestRequest::get()
            .uri("/lunar?solar_date=2024-12-10")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            "{\"data\":{\"lunar\":\"2024-11-10\",\"solar\":\"2024-12-10\",\"is_leap\":false}}"
        );
    }

    #[actix_web::test]
    async fn test_get_month_with_year() {
        let app = test::init_service(App::new().service(get_month_route)).await;
        let req = test::TestRequest::get()
            .uri("/dates?year=2024")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_month_with_year_and_month() {
        let app = test::init_service(App::new().service(get_month_route)).await;
        let req = test::TestRequest::get()
            .uri("/dates?year=2024&month=05")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}