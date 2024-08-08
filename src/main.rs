mod amlich;
mod vncalendar;

fn main() {
    let d = amlich::SolarDate::new(1, 10, 2024);
    println!("Hello, world!");
    println!("{}-{}-{}", d.day, d.month, d.year);
    let dd = vncalendar::time::VNDate::today();
    println!(
        "{}-{}-{}",
        dd.lunar_date.day, dd.lunar_date.month, dd.lunar_date.year
    );
    println!("{}", dd.time_zone_offset);
    print!("{}", dd)
}
