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