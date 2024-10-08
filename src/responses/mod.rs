use std::collections::HashMap;

use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use crate::models::{RequestEventId, VNDate};

#[derive(ToResponse, ToSchema, Serialize)]
pub struct ResponseMeta {
    request_event_id: Uuid,
}

impl ResponseMeta {
    pub fn new(request_event_id: RequestEventId) -> Self {
        Self {
            request_event_id: request_event_id.0,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        return Self { message };
    }
}

#[derive(ToResponse, ToSchema, Serialize)]
pub struct VNDateResponse {
    meta: Option<ResponseMeta>,
    data: VNDate,
}

impl VNDateResponse {
    pub fn new(data: VNDate) -> Self {
        Self { meta: None, data }
    }

    pub fn new_with_meta(data: VNDate, meta: ResponseMeta) -> Self {
        Self {
            meta: Some(meta),
            data,
        }
    }
}

#[derive(ToResponse, ToSchema, Serialize)]
pub struct YearDatesResponse {
    meta: ResponseMeta,
    data: HashMap<String, Vec<VNDate>>,
}

impl YearDatesResponse {
    pub fn new(data: HashMap<String, Vec<VNDate>>, meta: ResponseMeta) -> Self {
        Self { meta, data }
    }
}

#[derive(ToResponse, ToSchema, Serialize)]
pub struct YearMonthDatesResponse {
    meta: ResponseMeta,
    data: Vec<VNDate>,
}

impl YearMonthDatesResponse {
    pub fn new(data: Vec<VNDate>, meta: ResponseMeta) -> Self {
        Self { data, meta }
    }
}
