extern crate amlich;
extern crate vncalendar;

use std::collections::HashMap;

use actix_web::{get, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use utoipa::OpenApi;
use vncalendar::TIME_ZONE_OFFSET;

use crate::{
    models::VNDate,
    requests::{LunarToSolar, LunarToSolarDates},
    responses::{ErrorResponse, VNDateResponse, YearDatesResponse, YearMonthDatesResponse},
};

#[utoipa::path(
    get,
    path = "/today",
    responses(
        (status = 200, description = "Today in solar and lunar calendar", body = VNDateResponse),
    )
)]
#[get("/today")]
pub async fn today() -> HttpResponse {
    let t = vncalendar::time::VNDate::today();
    return HttpResponse::Ok().json(VNDateResponse::new(date_to_response(&t)));
}

#[utoipa::path(
    get,
    path = "/lunar",
    params(LunarToSolar),
    responses(
        (status = 200, description = "Convert solar date and lunar date", body = VNDateResponse),
    )
)]
#[get("/lunar")]
pub async fn to_lunar(solar: actix_web::web::Query<LunarToSolar>) -> HttpResponse {
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

#[utoipa::path(
    get,
    path = "/dates",
    params(LunarToSolarDates),
    responses(
        (status = 200, description = "List of all dates in given month", body = YearMonthDatesResponse),
    )
)]
#[get("/dates")]
pub async fn get_month(data: actix_web::web::Query<LunarToSolarDates>) -> HttpResponse {
    let year = data.year;
    if data.month != None {
        let month = data.month.unwrap();
        let res = vncalendar::get_month_dates(year, month);
        let mut dates_reponse: Vec<VNDate> = Vec::new();
        for date in res {
            dates_reponse.push(date_to_response(&date));
        }
        let response = YearMonthDatesResponse::new(dates_reponse);
        return HttpResponse::Ok().json(response);
    }

    let res = vncalendar::get_year_month_dates(year);
    let mut data: HashMap<String, Vec<VNDate>> = HashMap::new();
    for (m, dates) in res.iter() {
        let mut dates_reponse: Vec<VNDate> = Vec::new();
        for date in dates {
            dates_reponse.push(date_to_response(date));
        }
        data.insert(format!("{}", *m as isize), dates_reponse);
    }
    let response = YearDatesResponse::new(data);
    return HttpResponse::Ok().json(response);
}

#[derive(OpenApi)]
#[openapi(info(description = "Solar to Lunar date converter"))]
#[openapi(
    paths(today, to_lunar, get_month),
    components(schemas(VNDateResponse, YearDatesResponse, YearMonthDatesResponse, VNDate),)
)]
pub struct ApiDoc;

fn date_to_response(vndate: &vncalendar::time::VNDate) -> VNDate {
    let solar = format!("{}", vndate.get_solar_datetime().date_naive());
    let lunar = format!("{}", vndate.get_lunar_date());
    let is_leap = vndate.is_leap();
    return VNDate::new(lunar, solar, is_leap);
}
