use std::path::Path;

use cached::proc_macro::cached;

use crate::Answer;

fn part_1(input: &str) -> Option<i64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    stones
        .into_iter()
        .map(|stone| dp(stone, 25))
        .sum::<i64>()
        .into()
}

#[cached]
fn dp(stone: i64, remain: i64) -> i64 {
    if remain == 0 {
        return 1;
    }
    match stone {
        0 => dp(1, remain - 1),
        s if (s.ilog10() + 1) % 2 == 0 => {
            let mid = s.ilog10().div_ceil(2);
            let left = s / 10_i64.pow(mid);
            let right = s % 10_i64.pow(mid);
            dp(left, remain - 1) + dp(right, remain - 1)
        }
        _ => dp(stone * 2024, remain - 1),
    }
}

fn part_2(input: &str) -> Option<i64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    stones
        .into_iter()
        .map(|stone| dp(stone, 75))
        .sum::<i64>()
        .into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
