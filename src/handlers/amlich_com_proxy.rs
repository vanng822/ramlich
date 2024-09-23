extern crate amlich;
extern crate vncalendar;

use actix_web::{get, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::openapi::response;

#[derive(Debug, Serialize, Deserialize)]
struct AmLichCalendar {
    pub description: String,
    pub id: u32,
    pub name: String,
    ownerId: u32,
    privacy: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct AmLichCalendarResult {
    result: Vec<AmLichCalendar>,
    status: String,
}

#[utoipa::path(
    get,
    path = "/calendar",
    responses(
        (status = 200, description = "Convert solar date and lunar date", body = AmLichCalendar),
    )
)]
#[get("/calendar")]
pub async fn amlich_com_proxy(request: HttpRequest) -> HttpResponse {
    let result = reqwest::get("https://am-lich.com/api/web/v1/search").await;
    let res = match result {
        Ok(res) => res,
        _ => return HttpResponse::NotFound().into(),
    };
    let text_res = match res.text().await {
        Ok(text) => text,
        _ => return HttpResponse::NotFound().into(),
    };
    let response: AmLichCalendarResult = serde_json::from_str(&text_res).unwrap();

    return HttpResponse::Ok().json(response);
}
