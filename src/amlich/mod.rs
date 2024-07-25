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