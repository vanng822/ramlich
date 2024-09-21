extern crate amlich;
extern crate vncalendar;

use std::collections::HashMap;

use super::date_to_response;

use crate::{
    responses::{ResponseMeta, VNDateResponse},
    unleash::getunleash,
};
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use unleash_api_client::Context;
use uuid::{self, Uuid};

#[utoipa::path(
    get,
    path = "/today",
    responses(
        (status = 200, description = "Today in solar and lunar calendar", body = VNDateResponse),
    )
)]
#[get("/today")]
pub async fn today_route(request: HttpRequest) -> HttpResponse {
    let mut properties = HashMap::new();
    properties.insert("user_agent".to_string(), "Chrome".to_string());

    let context = Context {
        user_id: None,
        session_id: None,
        remote_address: None,
        environment: "development".to_string(),
        app_name: "apiserver".to_string(),
        properties: properties,
    };
    let default_feature =
        getunleash().get_variant(crate::unleash::UserFeatures::request_event, &context);
    info!("default_feature: {:#?}", default_feature);
    let request_event_id = request.extensions().get::<Uuid>().unwrap().clone();

    let t = vncalendar::time::VNDate::today();

    let response =
        VNDateResponse::new_with_meta(date_to_response(&t), ResponseMeta::new(request_event_id));

    return HttpResponse::Ok().json(response);
}
