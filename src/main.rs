extern crate amlich;
extern crate vncalendar;

use std::{borrow::Borrow, collections::HashMap};

use actix_web::{get, web, App, HttpResponse, HttpServer};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use vncalendar::{time::VNDate, Month, TIME_ZONE_OFFSET};

#[derive(Deserialize)]
struct LunarToSolar {
    solar_date: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct VNDateResponse {
    data: VNDate,
}

#[get("/today")]
async fn today() -> HttpResponse {
    let t = vncalendar::time::VNDate::today();
    return HttpResponse::Ok().json(VNDateResponse { data: t });
}

#[get("/lunar")]
async fn to_lunar(solar: actix_web::web::Query<LunarToSolar>) -> HttpResponse {
    let solar_date = match NaiveDate::parse_from_str(&solar.solar_date, "%Y-%m-%d") {
        Err(error) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: error.to_string(),
            })
        }
        Ok(solar) => solar,
    };

    let midday = NaiveDateTime::new(solar_date, NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    let t = vncalendar::time::VNDate::new(midday.and_utc(), TIME_ZONE_OFFSET);
    return HttpResponse::Ok().json(VNDateResponse { data: t });
}

#[derive(Deserialize)]
struct LunarToSolarDates {
    year: i32,
    month: Option<u32>,
}

#[derive(Serialize)]
struct YearDatesResponse {
    data: HashMap<String, Vec<VNDate>>,
}

#[derive(Serialize)]
struct YearMonthDatesResponse {
    data: Vec<VNDate>,
}

#[get("/dates")]
async fn get_month(data: actix_web::web::Query<LunarToSolarDates>) -> HttpResponse {
    let year = data.year;
    if data.month != None {
        let month = data.month.unwrap();
        let res = vncalendar::get_month_dates(year, month);
        let response = YearMonthDatesResponse { data: res };
        return HttpResponse::Ok().json(response);
    }

    let res = vncalendar::get_year_month_dates(year);
    let mut data: HashMap<String, Vec<VNDate>> = HashMap::new();
    for (m, dates) in res.iter() {
        data.insert(format!("{}", *m as isize), dates.to_vec());
    }
    let response = YearDatesResponse { data: data };
    return HttpResponse::Ok().json(response);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(today)
            .service(to_lunar)
            .service(get_month)
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test, App};
    use vncalendar::time::VNDate;

    use super::*;

    #[actix_web::test]
    async fn test_today_get() {
        let app = test::init_service(App::new().service(today)).await;
        let req = test::TestRequest::get().uri("/today").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_lunar_get() {
        let app = test::init_service(App::new().service(to_lunar)).await;
        let req = test::TestRequest::get()
            .uri("/lunar?solar_date=2024-12-10")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            "{\"data\":{\"lunar\":\"2024-11-10\",\"solar\":\"2024-12-10\",\"is_leap\":false}}"
        );
    }
}
