mod year_2023;

pub fn route(year: u16, day: u8) -> String {
    match year {
        2023 => match day {
            1 => year_2023::day_1::solve(),
            2 => year_2023::day_2::solve(),
            3 => year_2023::day_3::solve(),
            4 => year_2023::day_4::solve(),
            5 => year_2023::day_5::solve(),
            6 => year_2023::day_6::solve(),
            7 => year_2023::day_7::solve(),
            _ => format!("Not implemented yet for 2023 day {day}"),
        },
        _ => format!("Not implemented yet for year {year}"),
    }
}
