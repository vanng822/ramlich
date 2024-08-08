use std::{f32::consts::PI, fmt};

pub struct SolarDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

impl SolarDate {
    pub fn new(year: i64, month: i64, day: i64) -> SolarDate {
        SolarDate {
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

pub struct LunarDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
    pub is_leap: bool,
}

impl fmt::Display for LunarDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dstring = format!("{}-{:02}-{:02}", self.year, self.month, self.day);
        return write!(f, "{dstring}");
    }
}

impl LunarDate {
    pub fn new(year: i64, month: i64, day: i64, is_leap: bool) -> LunarDate {
        LunarDate {
            day: day,
            month: month,
            year: year,
            is_leap: is_leap,
        }
    }
}

fn jd_from_date(dd: i64, mm: i64, yyyy: i64) -> i64 {
    let a: i64 = ((14 - mm) / 12) as i64;
    let y = yyyy + 4800 - a;
    let m = mm + 12 * a - 3;
    let mut jd = dd + ((153 * m + 2) / 5) as i64 + 365 * y + (y / 4) as i64 - (y / 100) as i64
        + (y / 400) as i64
        - 32045;
    if jd < 2299161 {
        jd = dd + ((153 * m + 2) / 5) as i64 + 365 * y + (y / 4) as i64 - 32083;
    }
    return jd;
}

fn jd_to_date(jd: i64) -> SolarDate {
    let a: i64;
    let b: i64;
    let c: i64;

    if jd > 2299160 {
        // After 5/10/1582, Gregorian calendar
        a = jd + 32044;
        b = ((4 * a + 3) / 146097) as i64;
        c = a - ((b * 146097) / 4) as i64;
    } else {
        b = 0;
        c = jd + 32082;
    }

    let d = ((4 * c + 3) / 1461) as i64;
    let e = c - ((1461 * d) / 4) as i64;
    let m = ((5 * e + 2) / 153) as i64;
    let day = e - ((153 * m + 2) / 5) as i64 + 1;
    let month = m + 3 - 12 * ((m / 10) as i64);
    let year = b * 100 + d - 4800 + (m / 10) as i64;

    return SolarDate::new(year, month, day);
}

fn new_moon(ka: i64) -> f64 {
    let k = ka as f64;
    let T = k / 1236.85; // Time in Julian centuries from 1900 January 0.5
    let T2 = T * T;
    let T3 = T2 * T;
    let dr = (PI as f64) / 180.0;
    let mut Jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * T2 - 0.000000155 * T3;
    Jd1 = Jd1 + 0.00033 * ((166.56 + 132.87 * T - 0.009173 * T2) * dr).sin(); // Mean new moon
    let M = 359.2242 + 29.10535608 * k - 0.0000333 * T2 - 0.00000347 * T3; // Sun's mean anomaly
    let m_pr = 306.0253 + 385.81691806 * k + 0.0107306 * T2 + 0.00001236 * T3; // Moon's mean anomaly
    let F = 21.2964 + 390.67050646 * k - 0.0016528 * T2 - 0.00000239 * T3; // Moon's argument of latitude
    let mut C1 = (0.1734 - 0.000393 * T) * (M * dr).sin() + 0.0021 * (2.0 * dr * M).sin();
    C1 = C1 - 0.4068 * (m_pr * dr).sin() + 0.0161 * (dr * 2.0 * m_pr).sin();
    C1 = C1 - 0.0004 * (dr * 3.0 * m_pr).sin();
    C1 = C1 + 0.0104 * (dr * 2.0 * F).sin() - 0.0051 * (dr * (M + m_pr)).sin();
    C1 = C1 - 0.0074 * (dr * (M - m_pr)).sin() + 0.0004 * (dr * (2.0 * F + M)).sin();
    C1 = C1 - 0.0004 * (dr * (2.0 * F - M)).sin() - 0.0006 * (dr * (2.0 * F + m_pr)).sin();
    C1 = C1 + 0.0010 * (dr * (2.0 * F - m_pr)).sin() + 0.0005 * (dr * (2.0 * m_pr + M)).sin();
    let deltat: f64;
    if T < -11.0 {
        deltat = 0.001 + 0.000839 * T + 0.0002261 * T2 - 0.00000845 * T3 - 0.000000081 * T * T3;
    } else {
        deltat = -0.000278 + 0.000265 * T + 0.000262 * T2;
    };

    return Jd1 + C1 - deltat;
}

fn get_new_moon_day(k: i64, time_zone: i64) -> i64 {
    return (new_moon(k) + 0.5 + (time_zone as f64 / 24.0)) as i64;
}

fn get_lunar_month11(yyyy: i64, time_zone: i64) -> i64 {
    let off = jd_from_date(31, 12, yyyy) - 2415021;
    let k = (off as f64 / 29.530588853) as i64;
    let mut nm = get_new_moon_day(k, time_zone);
    let sun_long = get_sun_longitude(nm, time_zone); // sun longitude at local midnight
    if sun_long >= 9 {
        nm = get_new_moon_day(k - 1, time_zone);
    }
    return nm;
}

fn sun_longitude(jdn: f64) -> f64 {
    let T = (jdn - 2451545.0) / 36525.0; // Time in Julian centuries from 2000-01-01 12:00:00 GMT
    let T2 = T * T;
    let dr = PI as f64 / 180.0; // degree to radian
    let M = 357.52910 + 35999.05030 * T - 0.0001559 * T2 - 0.00000048 * T * T2; // mean anomaly, degree
    let L0 = 280.46645 + 36000.76983 * T + 0.0003032 * T2; // mean longitude, degree
    let mut DL = (1.914600 - 0.004817 * T - 0.000014 * T2) * (dr * M).sin();
    DL = DL + (0.019993 - 0.000101 * T) * (dr * 2.0 * M).sin() + 0.000290 * (dr * 3.0 * M).sin();
    let mut L = L0 + DL; // true longitude, degree
    L = L * dr;
    L = L - (PI as f64) * 2.0 * (((L / (PI as f64 * 2.0)) as i64) as f64); // Normalize to (0, 2*PI)

    return L;
}

fn get_sun_longitude(jd: i64, time_zone: i64) -> i64 {
    return (sun_longitude(jd as f64 - 0.5 - (time_zone as f64 / 24.0)) / PI as f64 * 6.0) as i64;
}

fn get_leap_month_offset(a11: i64, time_zone: i64) -> i64 {
    let k = ((a11 as f64 - 2415021.076998695) / 29.530588853 + 0.5) as i64;
    let mut last: i64;
    let mut i = 1; // We start with the month following lunar month 11
    let mut arc = get_sun_longitude(get_new_moon_day(k + i, time_zone), time_zone);

    loop {
        last = arc;
        i = i + 1;
        arc = get_sun_longitude(get_new_moon_day(k + i, time_zone), time_zone);

        if !(arc != last && i < 14) {
            break;
        }
    }

    return i - 1;
}

pub fn solar2lunar(solar_date: SolarDate, time_zone: i64) -> LunarDate {
    let yyyy = solar_date.year;
    let mm = solar_date.month;
    let dd = solar_date.day;

    let day_number = jd_from_date(dd, mm, yyyy);

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

    return LunarDate::new(lunar_year, lunar_month, lunar_day, is_leap);
}

pub fn lunar2solar(luna_date: LunarDate, time_zone: i64) -> SolarDate {
    let lunar_year = luna_date.year;
    let lunar_month = luna_date.month;
    let lunar_day = luna_date.day;
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
    fn jd_from_date_to_jd_to_date_test() {
        let result = jd_from_date(2, 2, 2024);
        assert_eq!(result, 2460343);
        let solar_date = jd_to_date(result);
        assert_eq!(solar_date.day, 2);
        assert_eq!(solar_date.month, 2);
        assert_eq!(solar_date.year, 2024);
    }

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
