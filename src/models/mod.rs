use derive_more::derive::Display;
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

#[derive(Display, Clone, Copy)]
pub struct RequestEventId(pub Uuid);

impl RequestEventId {
    pub fn new() -> Self {
        return Self(Uuid::new_v4());
    }
}

impl Into<Uuid> for RequestEventId {
    fn into(self) -> Uuid {
        return self.0;
    }
}
