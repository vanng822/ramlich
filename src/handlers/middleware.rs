use std::time::SystemTime;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{self, Next},
    Error, HttpMessage,
};
use log::{error, info};
use uuid::Uuid;

use crate::kafka::{KafkaProducer, RequestEvent};

pub async fn kafka_request_event_reporter(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let request_event_id = Uuid::new_v4();
    info!("kafka_request_event_reporter: {}", request_event_id);
    let start_time = SystemTime::now();

    let path = req.uri().to_string();
    // Inject request_event_id for endpoint to use in response
    req.extensions_mut().insert(request_event_id);

    let result = next.call(req).await;

    let response_time = SystemTime::now().duration_since(start_time);
    let request_event = RequestEvent {
        id: request_event_id,
        url: path,
        timestamp: SystemTime::now(),
        response_time: response_time.unwrap().as_millis() as u32,
    };
    let published_result = KafkaProducer::instance()
        .publish_request_event(&request_event)
        .await;

    if published_result != None {
        error!("Could not publish kafka event for request");
    }

    return result;
}
