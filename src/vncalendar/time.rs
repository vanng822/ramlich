use std::{fmt, ptr::null};

use crate::amlich;
use chrono::{DateTime, Datelike, Days, Months, Utc};
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

    pub fn with_solar_year(&self, year: i32) -> Option<VNDate> {
        let solar_time = self.solar_time.with_year(year);
        if solar_time == None {
            return None;
        }

        return Some(VNDate::new(solar_time.unwrap(), self.time_zone_offset));
    }

    pub fn with_solar_month(&self, month: u32) -> Option<VNDate> {
        let solar_time = self.solar_time.with_month(month);
        if solar_time == None {
            return None;
        }

        return Some(VNDate::new(solar_time.unwrap(), self.time_zone_offset));
    }

    pub fn with_solar_day(&self, day: u32) -> Option<VNDate> {
        let solar_time = self.solar_time.with_day(day);
        if solar_time == None {
            return None;
        }

        return Some(VNDate::new(solar_time.unwrap(), self.time_zone_offset));
    }

    pub fn add_solar_date(&self, years: u32, months: u32, days: u64) -> VNDate {
        let years_in_months = years * 12;
        let d = self.solar_time + Months::new(months + years_in_months) + Days::new(days);

        return VNDate::new(d, self.time_zone_offset);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_solar_date_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET).add_solar_date(1, 1, 1);
        assert_eq!(2023, result.solar_time.year());
        assert_eq!(10, result.solar_time.month());
        assert_eq!(12, result.solar_time.day());
    }
}
