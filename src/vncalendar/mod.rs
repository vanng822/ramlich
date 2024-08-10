use chrono::{DateTime, Datelike, Days, TimeZone, Utc};
use std::collections::HashMap;
use std::fmt;
use time::VNDate;

pub mod time;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub const MONTHS: [Month; 12] = [
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

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

pub fn get_year_month_dates(year: i32) -> HashMap<Month, Vec<VNDate>> {
    let mut result: HashMap<Month, Vec<VNDate>> = HashMap::new();
    for m in MONTHS {
        result.insert(m, get_month_dates(year, m as u32));
    }

    return result;
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

    #[test]
    fn get_year_month_dates_test() {
        let res = get_year_month_dates(2024);

        for (k, v) in res.iter() {
            println!("{}", *k);
            for d in v {
                println!("{}", d);
            }
        }

        assert_eq!(12, res.len());
        assert_eq!(31, res.get(&Month::January).unwrap().len());
        assert_eq!(29, res.get(&Month::February).unwrap().len());
        assert_eq!(31, res.get(&Month::March).unwrap().len());
        assert_eq!(30, res.get(&Month::April).unwrap().len());
        assert_eq!(31, res.get(&Month::May).unwrap().len());
        assert_eq!(30, res.get(&Month::June).unwrap().len());
        assert_eq!(31, res.get(&Month::July).unwrap().len());
        assert_eq!(31, res.get(&Month::August).unwrap().len());
        assert_eq!(30, res.get(&Month::September).unwrap().len());
        assert_eq!(31, res.get(&Month::October).unwrap().len());
        assert_eq!(30, res.get(&Month::November).unwrap().len());
        assert_eq!(31, res.get(&Month::December).unwrap().len());
    }
}
