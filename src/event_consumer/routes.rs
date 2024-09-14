use crate::event_consumer::db::get_request_event;
use actix_web::{get, web, HttpResponse};
use log::info;
use uuid::Uuid;

#[get("/request_event/{id}")]
pub async fn get_request_event_by_id(id: web::Path<Uuid>) -> HttpResponse {
    info!("request_event id: {}", id);
    let request_event = get_request_event(id.into_inner()).await.unwrap();
    return HttpResponse::Ok().json(request_event);
}
