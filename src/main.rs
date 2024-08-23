extern crate amlich;
extern crate vncalendar;

use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/today")]
async fn today() -> HttpResponse {
    let t = vncalendar::time::VNDate::today();
    return HttpResponse::Ok().json(t);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(today))
        .bind(("127.0.0.1", 8181))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(today)).await;
        let req = test::TestRequest::get().uri("/today").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
