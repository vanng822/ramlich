extern crate amlich;
extern crate vncalendar;

use std::time::SystemTime;

use super::date_to_response;

use crate::{
    kafka::{KafkaProducer, RequestEvent},
    models::RequestResult,
    responses::VNDateResponse,
};
use actix_web::{get, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::{self, Uuid};

#[utoipa::path(
    get,
    path = "/today",
    responses(
        (status = 200, description = "Today in solar and lunar calendar", body = VNDateResponse),
    )
)]
#[get("/today")]
pub async fn today_route(request: HttpRequest) -> HttpResponse {
    let producer = KafkaProducer::instance();

    let t = vncalendar::time::VNDate::today();
    let response = VNDateResponse::new(date_to_response(&t));

    let request_event = RequestResult {
        id: Uuid::new_v4(),
        url: request.uri().to_string(),
        timestamp: SystemTime::now(),
        response_time: 10,
    };
    let message = &RequestEvent::from(request_event);

    producer.publish_request_event(message).await;

    return HttpResponse::Ok().json(response);
}
