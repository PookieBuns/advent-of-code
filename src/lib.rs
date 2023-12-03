mod year_2023;

pub fn route(year: u16, day: u8) -> String {
    match year {
        2023 => match day {
            1 => year_2023::day_1::solve(),
            2 => year_2023::day_2::solve(),
            _ => format!("Not implemented yet for 2023 day {day}"),
        },
        _ => format!("Not implemented yet for year {year}"),
    }
}
