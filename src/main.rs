extern crate amlich;
extern crate vncalendar;

use std::{borrow::Borrow, collections::HashMap, fmt::format};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::{IntoParams, OpenApi, ToResponse, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use vncalendar::{Month, TIME_ZONE_OFFSET};

#[derive(IntoParams, Deserialize)]
struct LunarToSolar {
    solar_date: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(ToResponse, ToSchema, Serialize)]
struct VNDateResponse {
    data: VNDate,
}

#[derive(ToSchema, Serialize)]
struct VNDate {
    lunar: String,
    solar: String,
    is_leap: bool,
}

#[utoipa::path(
    get,
    path = "/today",
    responses(
        (status = 200, description = "Today in solar and lunar calendar", body = VNDateResponse),
    )
)]
#[get("/today")]
async fn today() -> HttpResponse {
    let t = vncalendar::time::VNDate::today();
    return HttpResponse::Ok().json(VNDateResponse {
        data: date_to_response(&t),
    });
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
    return HttpResponse::Ok().json(VNDateResponse {
        data: date_to_response(&t),
    });
}

#[derive(Deserialize, IntoParams)]
struct LunarToSolarDates {
    year: i32,
    month: Option<u32>,
}

#[derive(ToResponse, ToSchema, Serialize)]
struct YearDatesResponse {
    data: HashMap<String, Vec<VNDate>>,
}

#[derive(ToResponse, ToSchema, Serialize)]
struct YearMonthDatesResponse {
    data: Vec<VNDate>,
}

fn date_to_response(vndate: &vncalendar::time::VNDate) -> VNDate {
    return VNDate {
        solar: format!("{}", vndate.get_solar_datetime().date_naive()),
        lunar: format!("{}", vndate.get_lunar_date()),
        is_leap: vndate.is_leap(),
    };
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
async fn get_month(data: actix_web::web::Query<LunarToSolarDates>) -> HttpResponse {
    let year = data.year;
    if data.month != None {
        let month = data.month.unwrap();
        let res = vncalendar::get_month_dates(year, month);
        let mut dates_reponse: Vec<VNDate> = Vec::new();
        for date in res {
            dates_reponse.push(date_to_response(&date));
        }
        let response = YearMonthDatesResponse {
            data: dates_reponse,
        };
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
    let response = YearDatesResponse { data: data };
    return HttpResponse::Ok().json(response);
}

#[derive(OpenApi)]
#[openapi(info(description = "Solar to Lunar date converter"))]
#[openapi(
    paths(today, to_lunar, get_month),
    components(schemas(VNDateResponse, YearDatesResponse, YearMonthDatesResponse, VNDate),)
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
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
