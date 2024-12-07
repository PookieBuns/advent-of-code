// mod year_2023;
mod year_2024;

pub struct Answer {
    pub part_1: Option<i64>,
    pub part_2: Option<i64>,
}

impl Answer {
    fn from_parts<T: Into<i64>>(part_1: Option<T>, part_2: Option<T>) -> Self {
        Self {
            part_1: part_1.map(Into::into),
            part_2: part_2.map(Into::into),
        }
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(part_1) = self.part_1 {
            writeln!(f, "Part 1: {}", part_1)?;
        } else {
            writeln!(f, "Part 1: Not implemented yet")?;
        }
        if let Some(part_2) = self.part_2 {
            writeln!(f, "Part 2: {}", part_2)?;
        } else {
            writeln!(f, "Part 2: Not implemented yet")?;
        }
        Ok(())
    }
}

pub fn route(year: i32, day: u32) -> Answer {
    match year {
        // 2023 => match day {
        //     1 => year_2023::day_1::solve(),
        //     2 => year_2023::day_2::solve(),
        //     3 => year_2023::day_3::solve(),
        //     4 => year_2023::day_4::solve(),
        //     5 => year_2023::day_5::solve(),
        //     6 => year_2023::day_6::solve(),
        //     7 => year_2023::day_7::solve(),
        //     8 => year_2023::day_8::solve(),
        //     _ => format!("Not implemented yet for 2023 day {day}"),
        // },
        2024 => match day {
            1 => year_2024::day_1::solve(),
            2 => year_2024::day_2::solve(),
            3 => year_2024::day_3::solve(),
            4 => year_2024::day_4::solve(),
            5 => year_2024::day_5::solve(),
            6 => year_2024::day_6::solve(),
            7 => year_2024::day_7::solve(),
            _ => unimplemented!("Not implemented yet for 2024 day {day}"),
        },
        _ => unimplemented!("Not implemented yet for year {year}"),
    }
}
