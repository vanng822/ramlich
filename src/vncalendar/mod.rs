pub mod time;

/*
pub fn GetMonthDates(year: i64, month: i64) -> Vec<VNDate> {
    let dates: Vec<VNDate>;

    let start: DateTime<Utc> = Utc
        .with_ymd_and_hms(year, month, 1, 12, 0, 0)
        .unwrap()
        .and_local_timezone(VietNamTimeZone);

    for i in 0..27 {
        d := FromSolarTime(start.AddDate(0, 0, i))
        dates = append(dates, d)
    }

    for i := 28; i < 31; i++ {
        d := FromSolarTime(start.AddDate(0, 0, i))
        // next month
        if d.SolarTime().Month() != month {
            break
        }
        dates = append(dates, d)
    }

    return dates;
}
*/
