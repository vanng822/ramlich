use std::time::SystemTime;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize)]
pub struct VNDate {
    lunar: String,
    solar: String,
    is_leap: bool,
}

impl VNDate {
    pub fn new(lunar: String, solar: String, is_leap: bool) -> Self {
        return Self {
            lunar: lunar,
            solar: solar,
            is_leap: is_leap,
        };
    }
}

pub trait Response {
    fn to_string(&self) -> String;
}

pub struct RequestResult {
    pub id: Uuid,
    pub url: String,
    pub timestamp: SystemTime,
    pub response_time: i32,
}
