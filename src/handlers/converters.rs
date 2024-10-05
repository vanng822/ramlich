extern crate vncalendar;
use amlich::LunarDate;
use chrono::{DateTime, FixedOffset};

use crate::models::VNDate;

pub fn date_to_response(vndate: &vncalendar::time::VNDate) -> VNDate {
    let solar = format!("{}", vndate.get_solar_datetime().date_naive());
    let lunar = format!("{}", vndate.get_lunar_date());
    let is_leap = vndate.is_leap();
    return VNDate::new(lunar, solar, is_leap);
}
