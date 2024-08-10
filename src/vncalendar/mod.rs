use chrono::{DateTime, Datelike, Days, TimeZone, Utc};
use time::VNDate;

pub mod time;

pub fn get_month_dates(year: i32, month: u32) -> Vec<VNDate> {
    let mut dates: Vec<VNDate> = vec![];

    let start: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, 1, 12, 0, 0).unwrap();

    for i in 0..28 {
        let solar_date = start.checked_add_days(Days::new(i)).unwrap();
        dates.push(VNDate::new(solar_date, time::TIME_ZONE_OFFSET));
    }

    for i in 28..31 {
        let solar_date = start.checked_add_days(Days::new(i)).unwrap();
        let d = VNDate::new(solar_date, time::TIME_ZONE_OFFSET);
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
        assert_eq!(31, get_month_dates(2016, 7).len());
        assert_eq!(29, get_month_dates(2016, 2).len());
        assert_eq!(28, get_month_dates(2017, 2).len());
        assert_eq!(29, get_month_dates(2024, 2).len());
    }
}
