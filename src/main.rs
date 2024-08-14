extern crate amlich;

mod vncalendar;

fn main() {
    let d = amlich::SolarDate::new(1, 10, 2024);
    println!("{}-{}-{}", d.day, d.month, d.year);
    let dd = vncalendar::time::VNDate::today();
    println!("{}-{}-{}", dd.year(), dd.month(), dd.day());
    println!("{}", dd);
    println!("{}", dd.with_solar_year(2028).unwrap());
    println!("{}", dd.with_solar_month(10).unwrap());
    println!("{}", dd.with_solar_day(28).unwrap());
    println!("{}", dd.add_solar_date(1, 0, 0));
    let dd2 = vncalendar::time::VNDate::new(dd.get_solar_datetime(), vncalendar::TIME_ZONE_OFFSET);
    assert!(dd == dd2);
}
