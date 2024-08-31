extern crate amlich;
extern crate vncalendar;

use super::date_to_response;

use crate::responses::VNDateResponse;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    get,
    path = "/today",
    responses(
        (status = 200, description = "Today in solar and lunar calendar", body = VNDateResponse),
    )
)]
#[get("/today")]
pub async fn today_route() -> HttpResponse {
    let t = vncalendar::time::VNDate::today();
    return HttpResponse::Ok().json(VNDateResponse::new(date_to_response(&t)));
}
