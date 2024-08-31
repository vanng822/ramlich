use serde::Deserialize;
use utoipa::IntoParams;

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct LunarToSolar {
    // Assume 4 digits year
    #[param(max_length = 10)]
    pub solar_date: String,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct LunarToSolarDates {
    #[param()]
    pub year: i32,
    #[param()]
    pub month: Option<u32>,
}
