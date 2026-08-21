// Round trip solar -> lunar -> solar for every day from 1800-01-01 to 2100-12-31.
use amlich::{lunar2solar, solar2lunar, LunarDate, SolarDate};

#[test]
fn round_trip_1800_2100() {
    let mut date = SolarDate::new(1800, 1, 1);
    let mut days = 0;
    loop {
        let lunar = solar2lunar(date, 7);
        let back = lunar2solar(
            LunarDate::new(lunar.year, lunar.month, lunar.day, lunar.is_leap),
            7,
        );
        assert!(
            back.year == date.year && back.month == date.month && back.day == date.day,
            "round trip failed: {date} -> {lunar} -> {back}"
        );
        days += 1;

        if date.year == 2100 && date.month == 12 && date.day == 31 {
            break;
        }
        date = next_day(date);
    }
    assert!(days > 100_000, "only checked {days} days");
}

fn next_day(d: SolarDate) -> SolarDate {
    let last = days_in_month(d.year, d.month);
    if d.day < last {
        SolarDate::new(d.year, d.month, d.day + 1)
    } else if d.month < 12 {
        SolarDate::new(d.year, d.month + 1, 1)
    } else {
        SolarDate::new(d.year + 1, 1, 1)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
    }
}
