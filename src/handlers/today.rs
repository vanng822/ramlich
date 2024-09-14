extern crate amlich;
extern crate vncalendar;

use std::time::SystemTime;

use super::date_to_response;

use crate::{
    kafka::{KafkaProducer, RequestEvent},
    models::RequestResult,
    responses::ResponseMeta,
    responses::VNDateResponse,
};
use actix_web::{get, HttpRequest, HttpResponse};
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

    let reqquest_event_id = Uuid::new_v4();

    let request_event = RequestResult {
        id: reqquest_event_id,
        url: request.uri().to_string(),
        timestamp: SystemTime::now(),
        response_time: 10,
    };
    let message = &RequestEvent::from(request_event);

    let published_result = producer.publish_request_event(message).await;

    let response = if published_result != None {
        VNDateResponse::new(date_to_response(&t));
    } else {
        VNDateResponse::new_with_meta(date_to_response(&t), ResponseMeta::new(reqquest_event_id));
    };

    return HttpResponse::Ok().json(response);
}
