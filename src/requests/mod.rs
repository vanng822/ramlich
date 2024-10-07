use serde::Deserialize;
use utoipa::IntoParams;

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct SolarToLunar {
    // Assume 4 digits year
    #[param(max_length = 10)]
    pub solar_date: String,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SolarToLunarDates {
    #[param()]
    pub year: i32,
    #[param()]
    pub month: Option<u8>,
}
