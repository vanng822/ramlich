use std::f32::consts::PI;

pub struct SolarDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

impl SolarDate {
    pub fn new(day: i32, month: i32, year: i32) -> SolarDate {
        SolarDate{day: day, month: month, year: year}
    }
}

pub struct LunarDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap: bool,
}

impl LunarDate {
    pub fn new(day: i32, month: i32, year: i32, is_leap: bool) -> LunarDate {
        LunarDate{day: day, month: month, year: year, is_leap: is_leap}
    }
}

pub fn jd_from_date(dd: i32, mm: i32, yyyy: i32) -> i32 {
	let a: i32 = (14 - mm) / 12;
	let y = yyyy + 4800 - a;
	let m = mm + 12 * a - 3;
	let mut jd = dd + ((153 * m + 2) / 5) + 365 * y + (y / 4) - (y / 100) + (y / 400) - 32045;
	if jd < 2299161 {
		jd = dd + ((153 * m + 2) / 5) + 365 * y + (y / 4) - 32083;
	}
	return jd;
}

pub fn jd_to_date(jd: i32) -> SolarDate {
    let a: i32;
    let b: i32;
    let c: i32;

    if jd > 2299160 { // After 5/10/1582, Gregorian calendar
        a = jd + 32044;
        b = (4 * a + 3) / 146097;
        c = a - ((b * 146097) / 4);
    } else {
        b = 0;
        c = jd + 32082;
    }

    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - ((153 * m + 2) / 5) + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = b * 100 + d - 4800 + (m / 10);

    let date = SolarDate::new(day, month, year);

	return date;
}


pub fn new_moon(ka: i32) -> f64 {
    let k = ka as f64;
	let T = k / 1236.85; // Time in Julian centuries from 1900 January 0.5
	let T2 = T * T;
	let T3 = T2 * T;
	let dr = (PI as f64) / 180.0;
	let Jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * T2 - 0.000000155 * T3;
	let Jd1 = Jd1 + 0.00033 * ((166.56 + 132.87 * T - 0.009173 * T2) * dr).sin(); // Mean new moon
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
		deltat = 0.001 + 0.000839 * T + 0.0002261 * T2 - 0.00000845 * T3
				- 0.000000081 * T * T3;
	} else {
		deltat = -0.000278 + 0.000265 * T + 0.000262 * T2;
	};
	let jd_new = Jd1 + C1 - deltat;
	return jd_new;
}

fn get_new_moon_day(k: i32, time_zone: i32) -> i32 {
	return (new_moon(k) + 0.5 + time_zone as f64 / 24.0) as i32;
}

fn get_lunar_month11(yyyy: i32, time_zone: i32) -> i32 {
	let off = jd_from_date(31, 12, yyyy) - 2415021;
	let k = (off as f64 / 29.530588853) as i32;
	let mut nm = get_new_moon_day(k, time_zone);
	let sun_long = get_sun_longitude(nm, time_zone); // sun longitude at local midnight
	if sun_long >= 9 {
		nm = get_new_moon_day(k - 1, time_zone);
	}
	return nm;
}

pub fn sun_longitude(jdn: f64) -> f64 {
	let T = (jdn - 2451545.0) / 36525.0; // Time in Julian centuries from 2000-01-01 12:00:00 GMT
	let T2 = T * T;
	let dr = (PI / 180.0) as f64; // degree to radian
	let M = 357.52910 + 35999.05030 * T - 0.0001559 * T2 - 0.00000048 * T * T2; // mean anomaly, degree
	let L0 = 280.46645 + 36000.76983 * T + 0.0003032 * T2; // mean longitude, degree
	let DL = (1.914600 - 0.004817 * T - 0.000014 * T2) * (dr * M).sin();
	let DL = DL + (0.019993 - 0.000101 * T) * (dr * 2.0 * M).sin() + 0.000290 * (dr * 3.0 * M).sin();
	let mut L = L0 + DL; // true longitude, degree
	L = L * dr;
	L = L - (PI as f64) * 2.0 * (((L / (PI as f64 * 2.0)) as i64) as f64); // Normalize to (0, 2*PI)
	
    return L;
}

pub fn get_sun_longitude(jd: i32, time_zone: i32) -> i32 {
	return (sun_longitude(jd as f64 - 0.5 - (time_zone as f64 / 24.0)) / (PI as f64 * 6.0)) as i32;
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
}