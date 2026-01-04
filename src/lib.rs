// mod year_2023;
mod year_2024;
mod year_2025;

pub struct Answer {
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}

impl Answer {
    fn from_parts<T: std::string::ToString, U: std::string::ToString>(
        part_1: Option<T>,
        part_2: Option<U>,
    ) -> Answer {
        Answer {
            part_1: part_1.map(|p| p.to_string()),
            part_2: part_2.map(|p| p.to_string()),
        }
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(part_1) = self.part_1.as_ref() {
            writeln!(f, "Part 1: {}", part_1)?;
        } else {
            writeln!(f, "Part 1: Not implemented yet")?;
        }
        if let Some(part_2) = self.part_2.as_ref() {
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
            8 => year_2024::day_8::solve(),
            9 => year_2024::day_9::solve(),
            10 => year_2024::day_10::solve(),
            11 => year_2024::day_11::solve(),
            12 => year_2024::day_12::solve(),
            13 => year_2024::day_13::solve(),
            14 => year_2024::day_14::solve(),
            15 => year_2024::day_15::solve(),
            16 => year_2024::day_16::solve(),
            17 => year_2024::day_17::solve(),
            18 => year_2024::day_18::solve(),
            19 => year_2024::day_19::solve(),
            20 => year_2024::day_20::solve(),
            21 => year_2024::day_21::solve(),
            22 => year_2024::day_22::solve(),
            23 => year_2024::day_23::solve(),
            24 => year_2024::day_24::solve(),
            25 => year_2024::day_25::solve(),
            _ => unimplemented!("Not implemented yet for 2024 day {day}"),
        },
        2025 => match day {
            1 => year_2025::day_1::solve(),
            2 => year_2025::day_2::solve(),
            3 => year_2025::day_3::solve(),
            4 => year_2025::day_4::solve(),
            5 => year_2025::day_5::solve(),
            6 => year_2025::day_6::solve(),
            7 => year_2025::day_7::solve(),
            8 => year_2025::day_8::solve(),
            9 => year_2025::day_9::solve(),
            10 => year_2025::day_10::solve(),
            11 => year_2025::day_11::solve(),
            // 12 => year_2025::day_12::solve(),
            _ => unimplemented!("Not implemented yet for 2025 day {day}"),
        },
        _ => unimplemented!("Not implemented yet for year {year}"),
    }
}
