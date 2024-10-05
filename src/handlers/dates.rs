extern crate amlich;
extern crate vncalendar;

use super::converters::date_to_response;

use std::collections::HashMap;

use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
use vncalendar::Month;

use crate::{
    models::{RequestEventId, VNDate},
    requests::SolarToLunarDates,
    responses::{ErrorResponse, ResponseMeta, YearDatesResponse, YearMonthDatesResponse},
};

#[utoipa::path(
    get,
    path = "/dates",
    params(SolarToLunarDates),
    responses(
        (status = 200, description = "List of all dates in given month", body = YearMonthDatesResponse),
    )
)]
#[get("/dates")]
pub async fn get_month_route(
    request: HttpRequest,
    data: actix_web::web::Query<SolarToLunarDates>,
) -> HttpResponse {
    let request_event_id = request
        .extensions()
        .get::<RequestEventId>()
        .unwrap()
        .clone();
    let year = data.year;
    if data.month != None {
        let month = Month::try_from(data.month.unwrap());

        if month.is_err() {
            return HttpResponse::BadRequest()
                .json(ErrorResponse::new(month.unwrap_err().to_string()));
        }

        let res = vncalendar::get_month_dates(year, month.unwrap());
        let mut dates_reponse: Vec<VNDate> = Vec::new();
        for date in res {
            dates_reponse.push(date_to_response(&date));
        }
        let response =
            YearMonthDatesResponse::new(dates_reponse, ResponseMeta::new(request_event_id));
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
    let response = YearDatesResponse::new(data, ResponseMeta::new(request_event_id));
    return HttpResponse::Ok().json(response);
}
