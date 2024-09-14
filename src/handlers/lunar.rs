extern crate amlich;
extern crate vncalendar;

use super::converters::date_to_response;

use actix_web::{get, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use vncalendar::TIME_ZONE_OFFSET;

use crate::{
    requests::SolarToLunar,
    responses::{ErrorResponse, VNDateResponse},
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
pub async fn lunar_route(solar: actix_web::web::Query<SolarToLunar>) -> HttpResponse {
    let solar_date = match NaiveDate::parse_from_str(&solar.solar_date, "%Y-%m-%d") {
        Err(error) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(error.to_string()));
        }
        Ok(solar) => solar,
    };

    let midday = NaiveDateTime::new(solar_date, NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    let t = vncalendar::time::VNDate::new(midday.and_utc(), TIME_ZONE_OFFSET);
    return HttpResponse::Ok().json(VNDateResponse::new(date_to_response(&t)));
}
