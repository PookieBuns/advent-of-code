use std::path::Path;

use crate::Answer;

#[derive(Debug)]
struct Schematic {
    schema: Schema,
    pins: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
enum Schema {
    Lock,
    Key,
}

fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn convert_to_schematic(grid: Vec<Vec<char>>) -> Schematic {
    let top_row_all_hashes = grid[0].iter().all(|&c| c == '#');
    let schema = if top_row_all_hashes {
        Schema::Key
    } else {
        Schema::Lock
    };
    let pins = match schema {
        Schema::Key => (0..grid[0].len())
            .map(|j| grid.iter().position(|row| row[j] == '.').unwrap())
            .collect::<Vec<_>>(),
        Schema::Lock => (0..grid[0].len())
            .map(|j| grid.iter().rev().position(|row| row[j] == '.').unwrap())
            .collect::<Vec<_>>(),
    };
    Schematic { schema, pins }
}

fn part_1(input: &str) -> Option<i32> {
    let schematics = input
        .split("\n\n")
        .map(convert_to_grid)
        .map(convert_to_schematic)
        .collect::<Vec<_>>();
    let lock_schematics = schematics
        .iter()
        .filter(|schematic| schematic.schema == Schema::Lock)
        .collect::<Vec<_>>();
    let key_schematics = schematics
        .iter()
        .filter(|schematic| schematic.schema == Schema::Key)
        .collect::<Vec<_>>();
    (lock_schematics
        .iter()
        .map(|lock_schematic| {
            key_schematics
                .iter()
                .filter(|key_schematic| {
                    std::iter::zip(&lock_schematic.pins, &key_schematic.pins)
                        .all(|(lock_pin, key_pin)| lock_pin + key_pin <= 7)
                })
                .count()
        })
        .sum::<usize>() as i32)
        .into()
}

fn part_2(_input: &str) -> Option<i32> {
    None
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1_start_time = std::time::Instant::now();
    let part_1 = part_1(&input);
    let part_1_elapsed_time = part_1_start_time.elapsed();
    println!("Done with part 1 in {:?}", part_1_elapsed_time);
    let part_2_start_time = std::time::Instant::now();
    let part_2 = part_2(&input);
    let part_2_elapsed_time = part_2_start_time.elapsed();
    println!("Done with part 2 in {:?}", part_2_elapsed_time);
    Answer::from_parts(part_1, part_2)
}
