use std::fmt;

use crate::amlich;
use chrono::{DateTime, Datelike, Utc};
const TIME_ZONE_OFFSET: i64 = 7;

pub struct VNDate {
    pub solar_time: DateTime<Utc>,
    pub lunar_date: amlich::LunarDate,
    pub time_zone_offset: i64,
}

impl VNDate {
    pub fn new(solar_time: DateTime<Utc>, time_zone_offset: i64) -> VNDate {
        let lunar_date = amlich::solar2lunar(
            amlich::SolarDate::new(
                solar_time.year() as i64,
                solar_time.month() as i64,
                solar_time.day() as i64,
            ),
            time_zone_offset,
        );

        return VNDate {
            solar_time: solar_time,
            time_zone_offset: time_zone_offset,
            lunar_date: lunar_date,
        };
    }

    pub fn today() -> VNDate {
        return VNDate::new(Utc::now(), TIME_ZONE_OFFSET);
    }
}

impl fmt::Display for VNDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{lunar} => {solar}",
            lunar = self.lunar_date,
            solar = self.solar_time.date_naive()
        );
    }
}
