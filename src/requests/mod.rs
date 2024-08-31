use serde::Deserialize;
use utoipa::IntoParams;

#[derive(IntoParams, Deserialize)]
pub struct LunarToSolar {
    pub solar_date: String,
}

#[derive(Deserialize, IntoParams)]
pub struct LunarToSolarDates {
    pub year: i32,
    pub month: Option<u32>,
}
