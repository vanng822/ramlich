extern crate amlich;
use std::fmt::{self};

use chrono::{DateTime, Datelike, Days, Duration, Months, TimeDelta, Utc};

use super::TIME_ZONE_OFFSET;

pub struct VNDate {
    solar_time: DateTime<Utc>,
    lunar_date: amlich::LunarDate,
    time_zone_offset: i64,
}

const standard_error: &str = "Invalid date format, should be similar as yyyy-mm-dd or %y-%m-%d";

impl VNDate {
    pub fn new(solar_time: DateTime<Utc>, time_zone_offset: i64) -> Self {
        let lunar_date = amlich::solar2lunar(
            amlich::SolarDate::new(solar_time.year(), solar_time.month(), solar_time.day()),
            time_zone_offset,
        );

        return Self {
            solar_time: solar_time,
            time_zone_offset: time_zone_offset,
            lunar_date: lunar_date,
        };
    }

    pub fn checked_add_signed(&self, rhs: TimeDelta) -> Option<VNDate> {
        let solar_time = self.solar_time.checked_add_signed(rhs);
        if solar_time == None {
            return None;
        }
        return Some(VNDate::new(solar_time.unwrap(), self.time_zone_offset));
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

    pub fn add(&self, duration: Duration) -> VNDate {
        let d = self.solar_time + duration;

        return VNDate::new(d, self.time_zone_offset);
    }

    pub fn equal(&self, other: &VNDate) -> bool {
        return self.solar_time.eq(&other.solar_time);
    }

    pub const fn get_lunar_date(&self) -> amlich::LunarDate {
        return self.lunar_date;
    }

    pub const fn get_solar_datetime(&self) -> DateTime<Utc> {
        return self.solar_time;
    }

    pub fn solar_day(&self) -> u32 {
        return self.solar_time.day();
    }

    pub fn solar_month(&self) -> u32 {
        return self.solar_time.month();
    }

    pub fn solar_year(&self) -> i32 {
        return self.solar_time.year();
    }

    pub fn day(&self) -> u32 {
        return self.lunar_date.day as u32;
    }

    pub fn month(&self) -> u32 {
        return self.lunar_date.month as u32;
    }

    pub fn year(&self) -> i32 {
        return self.lunar_date.year as i32;
    }

    pub fn is_leap(&self) -> bool {
        return self.lunar_date.is_leap;
    }

    pub fn format(&self, fmt: Option<&str>) -> Result<String, &str> {
        let s = fmt.unwrap_or("");

        if s != "" {
            let mut parts: Vec<&str> = s.split('-').collect();
            let mut separator: &str = "-";
            if parts.len() != 3 {
                parts = s.split('/').collect();
                if parts.len() != 3 {
                    return Err(standard_error);
                }
                separator = "/";
            }

            let first = parts[0];
            let second = parts[1];
            let third = parts[2];

            let mut correct_syntax = false;
            let mut reverse = false;

            match first {
                "yyyy" => correct_syntax = second == "mm" && third == "dd",
                "%y" => correct_syntax = second == "%m" && third == "%d",
                "dd" => {
                    correct_syntax = second == "mm" && third == "yyyy";
                    reverse = true;
                }
                "%d" => {
                    correct_syntax = second == "%m" && third == "%y";
                    reverse = true;
                }
                _ => correct_syntax = false,
            };

            if !correct_syntax {
                return Err(standard_error);
            }

            return match reverse {
                true => Ok(format!(
                    "{1:02}{0}{2:02}{0}{3:02}",
                    separator,
                    self.day(),
                    self.month(),
                    self.year()
                )),
                false => Ok(format!(
                    "{1}{0}{2:02}{0}{3:02}",
                    separator,
                    self.year(),
                    self.month(),
                    self.day()
                )),
            };
        }

        return Ok(format!(
            "{}-{:02}-{:02}",
            self.year(),
            self.month(),
            self.day()
        ));
    }

    pub fn today() -> VNDate {
        return VNDate::new(Utc::now(), TIME_ZONE_OFFSET);
    }
}

impl PartialEq for VNDate {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
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
    fn equal_op_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);

        let d1 = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        let d2 = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        assert!(d1 == d2);
        let d3 = VNDate::new(solar_time, TIME_ZONE_OFFSET).add_solar_date(1, 7, 40);
        assert!(d1 != d3);
    }

    #[test]
    fn add_solar_date_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET).add_solar_date(1, 7, 40);
        assert_eq!(2024, result.solar_time.year());
        assert_eq!(5, result.solar_time.month());
        assert_eq!(21, result.solar_time.day());
    }

    #[test]
    fn add_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET).add(TimeDelta::days(10));
        assert_eq!(2022, result.solar_time.year());
        assert_eq!(9, result.solar_time.month());
        assert_eq!(21, result.solar_time.day());
    }

    #[test]
    fn with_solar_year_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET)
            .with_solar_year(2024)
            .unwrap();
        assert_eq!(2024, result.solar_time.year());
        assert_eq!(9, result.solar_time.month());
        assert_eq!(11, result.solar_time.day());
    }

    #[test]
    fn with_solar_month_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET)
            .with_solar_month(7)
            .unwrap();
        assert_eq!(2022, result.solar_time.year());
        assert_eq!(7, result.solar_time.month());
        assert_eq!(11, result.solar_time.day());
    }

    #[test]
    fn with_solar_day_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let result = VNDate::new(solar_time, TIME_ZONE_OFFSET)
            .with_solar_day(22)
            .unwrap();
        assert_eq!(2022, result.solar_time.year());
        assert_eq!(9, result.solar_time.month());
        assert_eq!(22, result.solar_time.day());
    }

    #[test]
    fn equal_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let d = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        let other = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        assert_eq!(true, d.equal(&other));
    }

    #[test]
    fn checked_add_signed_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let d = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        // one day in seconds
        let rhs = TimeDelta::new(86_400, 0).unwrap();
        let result = d.checked_add_signed(rhs).unwrap();
        assert_eq!(12, result.solar_time.day());
    }

    #[test]
    fn format_test() {
        // Sun, 11 Sep 2022 18:34:48 UTC
        let nanos: i64 = 1662921288_000_000_000;
        let solar_time = DateTime::from_timestamp_nanos(nanos);
        let d = VNDate::new(solar_time, TIME_ZONE_OFFSET);
        assert_eq!("2022-07-11".to_string(), d.format(Some("%y-%m-%d")));
        assert_eq!("2022-07-11".to_string(), d.format(Some("yyyy-mm-dd")));
        assert_eq!("11/07/2022".to_string(), d.format(Some("%d/%m/%y")));
        assert_eq!("2022/07/11".to_string(), d.format(Some("yyyy/mm/dd")));
        let error = d.format(Some("y-m-d"));
        assert_eq!(standard_error, error);
    }
}
