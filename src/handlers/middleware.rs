use std::{str::FromStr, thread};

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    middleware::Next,
    Error, HttpMessage,
};
use chrono::Utc;
use log::{error, info};
use uuid::Uuid;

use crate::{
    kafka::{KafkaProducer, RequestEvent},
    models::RequestEventId,
    unleash::getunleash,
};

pub async fn kafka_request_event_reporter(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let request_event_id = RequestEventId::from(Uuid::new_v4());

    info!(
        "kafka_request_event_reporter: {}, thread id: {:?}",
        request_event_id,
        thread::current().id()
    );
    let requested_at = Utc::now();

    let path = req.uri().to_string();
    // Inject request_event_id for endpoint to use in response
    req.extensions_mut().insert(request_event_id);

    let response = next.call(req).await?;

    let request_event_enabled = getunleash().is_enabled(
        crate::unleash::UserFeatures::request_event_enabled,
        None,
        true,
    );
    info!("request_event_enabled: {}", request_event_enabled);

    let published_result = if request_event_enabled {
        let response_time = Utc::now().signed_duration_since(requested_at);
        let request_event = RequestEvent {
            id: request_event_id.into(),
            url: path,
            requested_at: requested_at,
            response_time: response_time.num_nanoseconds().unwrap(),
            status_code: response.status().as_u16(),
        };

        KafkaProducer::instance()
            .publish_request_event(&request_event)
            .await
    } else {
        None
    };

    // None is OK
    if published_result.is_none() {
        let (response_req, res) = response.into_parts();
        let (mut res, body) = res.into_parts();
        let hdrs = res.headers_mut();
        hdrs.insert(
            HeaderName::from_str("X-Request-Event-Id").unwrap(),
            HeaderValue::from_str(request_event_id.to_string().as_str()).unwrap(),
        );
        let res = res.set_body(body);
        Ok(ServiceResponse::new(response_req, res))
    } else {
        error!("Could not publish kafka event for request");
        Ok(response)
    }
}
