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
        Self {
            lunar: lunar,
            solar: solar,
            is_leap: is_leap,
        }
    }
}

#[derive(Display, Clone, Copy)]
pub struct RequestEventId(pub Uuid);

impl Into<Uuid> for RequestEventId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for RequestEventId {
    fn from(value: Uuid) -> RequestEventId {
        Self(value)
    }
}
