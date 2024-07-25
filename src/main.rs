mod amlich;

fn main() {
    let d = amlich::SolarDate::new(1, 10, 2024);
    println!("Hello, world!");
    println!("{}-{}-{}", d.day, d.month, d.year);
}

