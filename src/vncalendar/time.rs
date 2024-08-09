use std::{fmt, ptr::null};

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

    pub fn with_solar_year(&self, year: i32) -> VNDate {
        let solar_time = self.solar_time.with_year(year).unwrap();

        return VNDate::new(solar_time, self.time_zone_offset);
    }

    pub fn with_solar_month(&self, month: u32) -> VNDate {
        let solar_time = self.solar_time.with_month(month).unwrap();

        return VNDate::new(solar_time, self.time_zone_offset);
    }

    pub fn with_solar_day(&self, day: u32) -> VNDate {
        let solar_time = self.solar_time.with_day(day).unwrap();

        return VNDate::new(solar_time, self.time_zone_offset);
    }

    pub fn today() -> VNDate {
        return VNDate::new(Utc::now(), TIME_ZONE_OFFSET);
    }
}

impl fmt::Display for VNDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{lunar} ({solar})",
            lunar = self.lunar_date,
            solar = self.solar_time.date_naive()
        );
    }
}
