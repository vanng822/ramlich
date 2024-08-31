extern crate amlich;
extern crate vncalendar;

use super::converters::date_to_response;

use std::collections::HashMap;

use actix_web::{get, HttpResponse};

use crate::{
    models::VNDate,
    requests::LunarToSolarDates,
    responses::{YearDatesResponse, YearMonthDatesResponse},
};

#[utoipa::path(
    get,
    path = "/dates",
    params(LunarToSolarDates),
    responses(
        (status = 200, description = "List of all dates in given month", body = YearMonthDatesResponse),
    )
)]
#[get("/dates")]
pub async fn get_month_route(data: actix_web::web::Query<LunarToSolarDates>) -> HttpResponse {
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
