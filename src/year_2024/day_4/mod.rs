use std::path::Path;

use crate::Answer;

fn part_1(_input: &str) -> Option<i32> {
    None
}

fn part_2(_input: &str) -> Option<i32> {
    None
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer { part_1, part_2 }
}
