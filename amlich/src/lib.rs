use std::fmt;
mod fns;
use fns::{get_leap_month_offset, get_lunar_month11, get_new_moon_day, jd_to_date};

#[derive(Copy, Clone)]
pub struct SolarDate {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl SolarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            day: day,
            month,
            year: year,
        }
    }
}

impl fmt::Display for SolarDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dstring = format!("{}-{:02}-{:02}", self.year, self.month, self.day);
        return write!(f, "{dstring}");
    }
}

#[derive(Copy, Clone)]
pub struct LunarDate {
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub is_leap: bool,
}

impl fmt::Display for LunarDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print_leap = if self.is_leap { "L" } else { "" };
        let dstring = format!(
            "{}-{:02}-{:02}{}",
            self.year, self.month, self.day, print_leap
        );
        return write!(f, "{dstring}");
    }
}

impl LunarDate {
    pub fn new(year: i32, month: u32, day: u32, is_leap: bool) -> Self {
        Self {
            day: day,
            month: month,
            year: year,
            is_leap: is_leap,
        }
    }
}

pub fn solar2lunar(solar_date: SolarDate, time_zone: i64) -> LunarDate {
    let yyyy = solar_date.year as i64;
    let mm = solar_date.month as i64;
    let dd = solar_date.day as i64;

    let day_number = fns::jd_from_date(dd, mm, yyyy);

    let k = ((day_number as f64 - 2415021.076998695) / 29.530588853) as i64;
    let mut month_start = get_new_moon_day(k + 1, time_zone);
    if month_start > day_number {
        month_start = get_new_moon_day(k, time_zone);
    }
    let mut a11 = get_lunar_month11(yyyy, time_zone);
    let mut b11 = a11;
    let mut lunar_year: i64;
    if a11 >= month_start {
        lunar_year = yyyy;
        a11 = get_lunar_month11(yyyy - 1, time_zone);
    } else {
        lunar_year = yyyy + 1;
        b11 = get_lunar_month11(yyyy + 1, time_zone);
    }
    let lunar_day = day_number - month_start + 1;
    let diff = ((month_start - a11) / 29) as i64;
    let mut is_leap = false;
    let mut lunar_month = diff + 11;

    if b11 - a11 > 365 {
        let leap_month_diff = get_leap_month_offset(a11, time_zone);
        if diff >= leap_month_diff {
            lunar_month = diff + 10;
            if diff == leap_month_diff {
                is_leap = true;
            }
        }
    }
    if lunar_month > 12 {
        lunar_month = lunar_month - 12;
    }
    if lunar_month >= 11 && diff < 4 {
        lunar_year = lunar_year - 1;
    }

    return LunarDate::new(
        lunar_year as i32,
        lunar_month as u32,
        lunar_day as u32,
        is_leap,
    );
}

pub fn lunar2solar(luna_date: LunarDate, time_zone: i64) -> SolarDate {
    let lunar_year = luna_date.year as i64;
    let lunar_month = luna_date.month as i64;
    let lunar_day = luna_date.day as i64;
    let lunar_leap: i64 = luna_date.is_leap.into();

    let a11: i64;
    let b11: i64;

    if lunar_month < 11 {
        a11 = get_lunar_month11(lunar_year - 1, time_zone);
        b11 = get_lunar_month11(lunar_year, time_zone);
    } else {
        a11 = get_lunar_month11(lunar_year, time_zone);
        b11 = get_lunar_month11(lunar_year + 1, time_zone);
    }
    let k = (0.5 + (a11 as f64 - 2415021.076998695) / 29.530588853) as i64;
    let mut off = lunar_month - 11;

    if off < 0 {
        off += 12;
    }
    if b11 - a11 > 365 {
        let leap_off = get_leap_month_offset(a11, time_zone);
        let mut leap_month = leap_off - 2;
        if leap_month < 0 {
            leap_month += 12;
        }

        if lunar_leap != 0 && lunar_month != leap_month {
            return SolarDate::new(0, 0, 0);
        } else if lunar_leap != 0 || off >= leap_off {
            off += 1;
        }
    }
    let month_start = get_new_moon_day(k + off, time_zone);

    return jd_to_date(month_start + lunar_day - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar2lunar_is_leap_true_test() {
        let test_date = SolarDate::new(2006, 9, 12);
        let result = solar2lunar(test_date, 7);
        assert_eq!(result.day, 20);
        assert_eq!(result.month, 7);
        assert_eq!(result.year, 2006);
        assert_eq!(result.is_leap, true);
    }

    #[test]
    fn solar2lunar_is_leap_false_test() {
        let test_date = SolarDate::new(2006, 8, 13);
        let result = solar2lunar(test_date, 7);
        assert_eq!(result.day, 20);
        assert_eq!(result.month, 7);
        assert_eq!(result.year, 2006);
        assert_eq!(result.is_leap, false);
    }

    #[test]
    fn solar2lunar_is_leap_true_case_2_test() {
        let test_date = SolarDate::new(2012, 6, 12);
        let result = solar2lunar(test_date, 7);
        assert_eq!(result.day, 23);
        assert_eq!(result.month, 4);
        assert_eq!(result.year, 2012);
        assert_eq!(result.is_leap, true);
    }

    #[test]
    fn solar2lunar_is_leap_false_case_2_test() {
        let test_date = SolarDate::new(2012, 5, 13);
        let result = solar2lunar(test_date, 7);
        assert_eq!(result.day, 23);
        assert_eq!(result.month, 4);
        assert_eq!(result.year, 2012);
        assert_eq!(result.is_leap, false);
    }

    #[test]
    fn lunar2solar_is_leap_true_test() {
        let test_date = LunarDate::new(2006, 7, 20, true);
        let result = lunar2solar(test_date, 7);
        assert_eq!(result.day, 12);
        assert_eq!(result.month, 9);
        assert_eq!(result.year, 2006);
    }

    #[test]
    fn lunar2solar_is_leap_false_test() {
        let test_date = LunarDate::new(2006, 7, 20, false);
        let result = lunar2solar(test_date, 7);
        assert_eq!(result.day, 13);
        assert_eq!(result.month, 8);
        assert_eq!(result.year, 2006);
    }

    #[test]
    fn lunar2solar_is_leap_true_case_2_test() {
        let test_date = LunarDate::new(2012, 4, 23, true);
        let result = lunar2solar(test_date, 7);
        assert_eq!(result.day, 12);
        assert_eq!(result.month, 6);
        assert_eq!(result.year, 2012);
    }

    #[test]
    fn lunar2solar_is_leap_false_case_2_test() {
        let test_date = LunarDate::new(2012, 4, 23, false);
        let result = lunar2solar(test_date, 7);
        assert_eq!(result.day, 13);
        assert_eq!(result.month, 5);
        assert_eq!(result.year, 2012);
    }
}
