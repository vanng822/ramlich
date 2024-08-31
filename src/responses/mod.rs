use std::collections::HashMap;

use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

use crate::models::VNDate;

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
    data: VNDate,
}

impl VNDateResponse {
    pub fn new(data: VNDate) -> Self {
        return Self { data };
    }
}

#[derive(ToResponse, ToSchema, Serialize)]
pub struct YearDatesResponse {
    data: HashMap<String, Vec<VNDate>>,
}

impl YearDatesResponse {
    pub fn new(data: HashMap<String, Vec<VNDate>>) -> Self {
        return Self { data };
    }
}

#[derive(ToResponse, ToSchema, Serialize)]
pub struct YearMonthDatesResponse {
    data: Vec<VNDate>,
}

impl YearMonthDatesResponse {
    pub fn new(data: Vec<VNDate>) -> Self {
        return Self { data };
    }
}