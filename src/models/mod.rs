use serde::Serialize;
use utoipa::ToSchema;

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
