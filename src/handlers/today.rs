extern crate amlich;
extern crate vncalendar;

use super::date_to_response;

use crate::{responses::ResponseMeta, responses::VNDateResponse};
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
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
    let request_event_id = request.extensions().get::<Uuid>().unwrap().clone();

    let t = vncalendar::time::VNDate::today();

    let response =
        VNDateResponse::new_with_meta(date_to_response(&t), ResponseMeta::new(request_event_id));

    return HttpResponse::Ok().json(response);
}
