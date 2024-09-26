extern crate amlich;
extern crate vncalendar;

use actix_web::{
    dev::PeerAddr,
    error, get,
    web::{self, Buf},
    Error, HttpRequest, HttpResponse,
};
use log::error;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use url::Url;
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
pub async fn amlich_com_calendar_proxy(request: HttpRequest) -> HttpResponse {
    let mut client = awc::Client::default();
    for (key, value) in request.headers().iter() {
        client.headers().unwrap().append(key.clone(), value.clone());
    }

    let result = client
        .get("https://am-lich.com/api/web/v1/search")
        .send()
        .await;

    let res = match result {
        Ok(mut r) => r.body().await,
        Err(err) => {
            error!("get error from am-lich.com: {:?}", err);
            return HttpResponse::NotFound().into();
        }
    };
    let text_bytes = match res {
        Ok(text) => text,
        Err(err) => {
            error!("get error from am-lich.com: {:?}", err);
            return HttpResponse::NotFound().into();
        }
    };

    let response: AmLichCalendarResult = serde_json::from_reader(text_bytes.reader()).unwrap();

    return HttpResponse::Ok().json(response);
}

pub async fn amlich_com_forward(
    req: HttpRequest,
    mut payload: web::Payload,
    method: actix_web::http::Method,
    peer_addr: Option<PeerAddr>,
) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::default();
    let path = req.uri().path();

    let mut new_url = Url::parse("https://am-lich.com").unwrap();
    new_url.set_path(path);
    new_url.set_query(req.uri().query());

    let (tx, rx) = mpsc::unbounded_channel();

    actix_web::rt::spawn(async move {
        while let Some(chunk) = payload.next().await {
            tx.send(chunk).unwrap();
        }
    });

    let forwarded_req = client
        .request(
            reqwest::Method::from_bytes(method.as_str().as_bytes()).unwrap(),
            new_url,
        )
        .body(reqwest::Body::wrap_stream(UnboundedReceiverStream::new(rx)));

    // TODO: This forwarded implementation is incomplete as it only handles the unofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = match peer_addr {
        Some(PeerAddr(addr)) => forwarded_req.header("x-forwarded-for", addr.ip().to_string()),
        None => forwarded_req,
    };

    let res = forwarded_req
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut client_resp =
        HttpResponse::build(actix_web::http::StatusCode::from_u16(res.status().as_u16()).unwrap());

    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.insert_header((
            actix_web::http::header::HeaderName::from_bytes(header_name.as_ref()).unwrap(),
            actix_web::http::header::HeaderValue::from_bytes(header_value.as_ref()).unwrap(),
        ));
    }

    Ok(client_resp.streaming(res.bytes_stream()))
}
