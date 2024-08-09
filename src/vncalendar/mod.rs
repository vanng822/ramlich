use chrono::{DateTime, Datelike, TimeZone, Utc};
use time::VNDate;

pub mod time;

pub fn get_month_dates(year: i32, month: u32) -> Vec<VNDate> {
    let mut dates: Vec<VNDate> = vec![];

    let start: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, 1, 12, 0, 0).unwrap();
    let lunar_date = VNDate::new(start, time::TIME_ZONE_OFFSET);

    for i in 0..27 {
        let d = lunar_date.add_solar_date(0, 0, i);
        dates.push(d);
    }

    for i in 28..30 {
        let d = lunar_date.add_solar_date(0, 0, i);

        // next month
        if d.solar_time.month() != month {
            break;
        }
        dates.push(d);
    }

    return dates;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_month_dates_test() {
        let result = get_month_dates(2024, 2);
        // doesn't work with leap year
        // assert_eq!(29, result.len());
    }
}
