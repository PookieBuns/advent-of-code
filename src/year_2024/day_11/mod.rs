use std::{collections::HashMap, path::Path};

use crate::Answer;

fn part_1(input: &str) -> Option<i64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|stone| dp(stone, 25, &mut cache))
        .sum::<i64>()
        .into()
}

fn dp(stone: i64, remain: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if remain == 0 {
        return 1;
    }
    if let Some(ans) = cache.get(&(stone, remain)) {
        return *ans;
    }
    let ans = match stone {
        0 => dp(1, remain - 1, cache),
        s if (s.ilog10() + 1) % 2 == 0 => {
            let mid = (s.ilog10() + 1) / 2;
            let left = s / 10_i64.pow(mid);
            let right = s % 10_i64.pow(mid);
            dp(left, remain - 1, cache) + dp(right, remain - 1, cache)
        }
        _ => dp(stone * 2024, remain - 1, cache),
    };
    cache.insert((stone, remain), ans);
    ans
}

fn part_2(input: &str) -> Option<i64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|stone| dp(stone, 75, &mut cache))
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
