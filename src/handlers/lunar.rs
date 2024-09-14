extern crate amlich;
extern crate vncalendar;

use super::converters::date_to_response;

use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;
use vncalendar::TIME_ZONE_OFFSET;

use crate::{
    requests::SolarToLunar,
    responses::{ErrorResponse, ResponseMeta, VNDateResponse},
};

#[utoipa::path(
    get,
    path = "/lunar",
    params(SolarToLunar),
    responses(
        (status = 200, description = "Convert solar date and lunar date", body = VNDateResponse),
    )
)]
#[get("/lunar")]
pub async fn lunar_route(
    request: HttpRequest,
    solar: actix_web::web::Query<SolarToLunar>,
) -> HttpResponse {
    let solar_date = match NaiveDate::parse_from_str(&solar.solar_date, "%Y-%m-%d") {
        Err(error) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(error.to_string()));
        }
        Ok(solar) => solar,
    };

    let request_event_id = request.extensions().get::<Uuid>().unwrap().clone();

    let midday = NaiveDateTime::new(solar_date, NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    let t = vncalendar::time::VNDate::new(midday.and_utc(), TIME_ZONE_OFFSET);
    return HttpResponse::Ok().json(VNDateResponse::new_with_meta(
        date_to_response(&t),
        ResponseMeta::new(request_event_id),
    ));
}
