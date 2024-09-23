extern crate amlich;
extern crate vncalendar;

use actix_web::{get, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct AmLichCalendar {
    pub description: String,
    pub id: u32,
    pub name: String,
    #[serde(alias = "ownerId")]
    pub owner_id: u32,
    pub privacy: u16,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct AmLichCalendarResult {
    result: Vec<AmLichCalendar>,
    status: String,
}

#[utoipa::path(
    get,
    path = "/calendar",
    responses(
        (status = 200, description = "Convert solar date and lunar date", body = AmLichCalendarResult),
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
